---
title: Idiomatic Tensorflow on Android
tags: [android, tensorflow, kotlin]
date: 2021-06-07
blurb: "If you’ve used TensorFlow Lite on Android before, chances are that you’ve had to deal with the tedious task of pre-processing data, working with `Float` arrays in a statically typed language or resize, transform, normalize, and do any of the other standard tasks required before the data is fit for consumption by the model..."
---
# Idiomatic TensorFlow on Android — Get started with the TensorFlow Support Library
![](https://miro.medium.com/max/2400/1*xjsEPItNww_khr8o2me-xg.png)

<hr />

## Working with data on Android is inconvenient!

If you’ve used TensorFlow Lite on Android before, chances are that you’ve had to deal with the tedious task of pre-processing data, working with `Float` arrays in a statically typed language or resize, transform, normalize, and do any of the other standard tasks required before the data is fit for consumption by the model.

Well, no more! The [TFLite support library](https://github.com/tensorflow/tensorflow/tree/master/tensorflow/lite/experimental/support/java) nightly is now available, and in this post, we’ll go over its usage and build a wrapper around a `tflite` model.

> _**Note**: A companion repository for this post is available [here](https://github.com/ATechnoHazard/object-detection-yolo). Follow along, or jump straight into the source!_

## Scope

This post is limited in scope to loading and creating a wrapper class around a `tflite` model; however, you can see a fully functional project in the repository linked above. The code is liberally commented and very straightforward.

If you still have any queries, please don’t hesitate to reach out to me and drop a comment. I’ll be glad to help you out.

## Setting up the project

We’re going to be deploying a [TFLite version](https://github.com/ATechnoHazard/yolo-tflite) of the popular YOLOv3 object detection model on an Android device. Without further ado, let’s jump into it.

Create a new project using Android Studio, name it anything you like, and wait for the initial gradle sync to complete. Next, we’ll install the dependencies.

### Adding dependencies
Add the following dependencies to your app-level build.gradle.

```gradle
// Permissions handling
implementation 'com.github.quickpermissions:quickpermissions-kotlin:0.4.0'

// Tensorflow lite
implementation 'org.tensorflow:tensorflow-lite:0.0.0-nightly'
implementation 'org.tensorflow:tensorflow-lite-support:0.0.0-nightly'

// CameraView
implementation 'com.otaliastudios:cameraview:2.6.2'
```

1. **Quick Permissions**: This is a great library to make granting permissions quick and easy.
2. **Tensorflow Lite**: This is the core TFLite library.
3. **Tensorflow Lite Support**: This is the support library we’ll be using to make our data-related tasks simpler.
4. **CameraView**: This is a library that provides a simple API for accessing the camera.

## Configuring the gradle project

Our project still needs a little more configuration before we’re ready for the code. In the app-level `build.gradle` file, add the following options under the `android` block.

The reason we need to add this is because we’ll be shipping the model inside our assets, which are compressed by default, which is problematic because compressed models cannot be loaded by the interpreter.

```gradle
aaptOptions {
    noCompress "tflite"
}
```

> _**Note:** After this initial configuration, run the `gradle` sync again to fetch all dependencies._

## Jumping into the code

First things first; we need a model to load. The one I used can be found [here](https://raw.githubusercontent.com/ATechnoHazard/yolo-tflite/master/models/detect.tflite). Place the model inside `app/src/main/assets`. This will enable us to load it at runtime.

The labels for detected objects can be found [here](https://raw.githubusercontent.com/ATechnoHazard/object-detection-yolo/master/app/src/main/assets/labelmap.txt). Place them in the same directory as the model.

> _**Warning:** If you plan to use your own custom models, a word of caution; the input and output shapes may not match the ones used in this project._

### Creating a wrapper class
We’re going to wrap our model and its associated methods inside a class called YOLO. The initial code is as follows.

```java
class YOLO(private val context: Context) {
   private val interpreter: Interpreter
      
   companion object {
        private const val MODEL_FILE = "detect.tflite"
        private const val LABEL_FILE = "labelmap.txt"
    }

    init {
        val options = Interpreter.Options()
        interpreter = Interpreter(FileUtil.loadMappedFile(context, MODEL_FILE), options)
    }
}
```

Let’s break this class down into its core functionality and behaviour.

1. First, upon being created, the class loads the model from the app `assets` through the `FileUtil` class provided by the support library.
2. Next, we have a class member. The `interpreter` is self-explanatory, it’s an instance of a TFLite interpreter.
3. Finally, we have some static variables. These are just the file names of the model and the labels inside our `assets`.

Moving on, let’s add a convenience method to load our labels from the `assets`.

```java
class YOLO(private val context: Context) {
   // other stuff
    
   // lazily load object labels
    private val labelList by lazy { loadLabelList(context.assets) }
    
    private fun loadLabelList(
        assetManager: AssetManager
    ): List<String> {
        val labelList = mutableListOf<String>()
        val reader =
            BufferedReader(InputStreamReader(assetManager.open(LABEL_FILE)))
        var line = reader.readLine()
        while (line != null) {
            labelList.add(line)
            line = reader.readLine()
        }
        reader.close()
        return labelList
    }
}
```

Here we’ve declared a method that loads the label file and lazily initialized a member var to the returned value.

Let’s get down to the brass tacks. We’re now going to define a method that takes in a bitmap, passes it into the model and returns the detected object classes.

```java
class YOLO(private val context: Context) {
      private val interpreter: Interpreter

    // lazily load object labels
    private val labelList by lazy { loadLabelList(context.assets) }

    // create image processor to resize image to input dimensions
    private val imageProcessor by lazy {
        ImageProcessor.Builder()
            .add(ResizeOp(300, 300, ResizeOp.ResizeMethod.BILINEAR))
            .build()
    }

    // create tensorflow representation of an image
    private val tensorImage by lazy { TensorImage(DataType.UINT8) }

    fun detectObjects(bitmap: Bitmap): List<String> {
        tensorImage.load(bitmap)

        // resize image using processor
        val processedImage = imageProcessor.process(tensorImage)

        // load image data into input buffer
        val inputbuffer = TensorBuffer.createFixedSize(intArrayOf(1, 300, 300, 3), DataType.UINT8)
        inputbuffer.loadBuffer(processedImage.buffer, intArrayOf(1, 300, 300, 3))

        // create output buffers
        val boundBuffer = TensorBuffer.createFixedSize(intArrayOf(1, 10, 4), DataType.FLOAT32)
        val classBuffer = TensorBuffer.createFixedSize(intArrayOf(1, 10), DataType.FLOAT32)
        val classProbBuffer = TensorBuffer.createFixedSize(intArrayOf(1, 10), DataType.FLOAT32)
        val numBoxBuffer = TensorBuffer.createFixedSize(intArrayOf(1), DataType.FLOAT32)

        // run interpreter
        interpreter.runForMultipleInputsOutputs(
            arrayOf(inputbuffer.buffer), mapOf(
                0 to boundBuffer.buffer,
                1 to classBuffer.buffer,
                2 to classProbBuffer.buffer,
                3 to numBoxBuffer.buffer
            )
        )

        // map and return classnames to detected categories
        return classBuffer.floatArray.map { labelList[it.toInt() + 1] }
    }
}
```

Whoa, that’s a wall of code! Let’s go through it and break it down.

We’ve declared some new lazily initialized variables; an `ImageProcessor` and a `TensorImage`. These are classes exposed by the support library, to make loading images and processing them much simpler.

As shown here, we can load a `bitmap` directly into the `TensorImage` and then pass it on to the `ImageProcessor` for further processing.

The `ImageProcessor` has several operations available, but the one we’ve used here is to resize our input images to 300 * 300. This is because our model’s input size requires a 300 * 300 image.

After processing the image, we create several `TensorBuffers`. These are representations of Tensors that we can manipulate and access easily. The shapes of these `TensorBuffers` is determined by the model. Take a look at the model summary to figure out the appropriate shapes.

We load the `TensorImage` into the input `TensorBuffer`, and then pass the input and output buffers into the interpreter.

> _**Note:** The YOLOv3 model has multiple outputs. This is the reason why we had to use multiple output buffers._

After running inference, the interpreter sets the internal `FloatArrays` of the output buffers. Right now, we’re only interested in the one that contains the predicted classes. Using the handy kotlin `map` function, we map labels to the numerical classes output by the model and return them.

This class can now be used by our application to run inference on a `bitmap`. How convenient!

## Conclusion
And that’s it! Compared to a project without using the support library; we’d have written much more code to resize the image, convert bitmaps to `float` arrays, allocate `float` arrays manually to store the output in, etc.

To find out more, visit the documentation [here](https://www.tensorflow.org/lite/inference_with_metadata/lite_support).
