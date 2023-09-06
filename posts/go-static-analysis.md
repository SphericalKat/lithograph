---
title: Static Analysis with Go: The First Steps
tags: [go, AST, programming, static analysis, linter, analyzer]
date: 2023-09-06
blurb: "Learn the fundamentals of writing linters with Go, offering a gentle introduction to the art of static analysis."
---

# Static Analysis in Go: First Steps

***************Static Analysis***************. The art of analyzing programs without executing them, often to look for unwanted behaviour and unidiomatic code.

Most IDEs and LSPs come with some form of static analysis. Many of the code linters out there perform some kind of static analysis. Code quality tools like Sonarqube rely on static analysis to find issues with your code.

With that in mind, it’s apparent that knowing how these work is a useful skill to have, and it’s not even that difficult to get started with!

In this post, we’ll use the Go programming language to build a simple analzyer.

# But, before we begin…

Let’s lay some groundwork first. These are just some of the things you should know before you embark on this endeavour. First, let’s learn about ASTs.

## Abstract Syntax Trees

One of the things you’ll become intimately familiar with when working with analyzers is the ******AST****** (Abstract Syntax Tree).

Abstract Syntax Trees (ASTs) are a way of representing the structure of code or data in a program. They are used by compilers and interpreters to transform source code into executable code, or to analyze data structures.

An AST is a tree-like data structure that represents the syntactic structure of the code or data, without including information about the specific characters used to write the code. Instead, it captures the essential structure of the code, including the relationships between different parts of the code and the order in which they appear. 

The main job of any static analysis tool is to traverse the AST and find problems.

![Visualization of a Go program’s AST ](https://file.notion.so/f/f/81f817ee-2991-4622-8f27-6bbbfc6cbcc1/562743f1-6325-4f6b-a0f3-5f822c4a3353/Untitled.png?id=3858003c-0ee8-4b3e-b86e-d3d15596f6b2&table=block&spaceId=81f817ee-2991-4622-8f27-6bbbfc6cbcc1&expirationTimestamp=1694066400000&signature=4W2FfdYXeJ_24JGcOwnnNhEtDQTUmEIgixRvzL0I1Ps&downloadName=Untitled.png)

Visualization of a Go program’s AST 

For a quick peek to dive into ASTs, you can check out [https://astexplorer.net/](https://astexplorer.net/), which supports multiple languages.

## Why Go?

You might be wondering why we picked Go, of all languages. One of the reasons is that Go has a well-maintained suite of analysis tools built by the Go team, available under the package [analysis](https://pkg.go.dev/golang.org/x/tools/go/analysis).

The other is that it’s a popular language with a decent amount of features that we can write lints for.

With that out of the way, let’s begin.

# Our first linter

Our first linter is going to do something very simple. We want to detect and report if the `cancel` function returned by `context.WithCancel` is unused.

```go
package main

import "context"

func main() {
	ctx, _ := context.WithCancel(context.Background())

	// just to discard ctx
	_ = ctx
}
```

Since what we want to analyze (a `context.WithCancel` call) will always be inside a function, we can focus on this part of the AST on [astexplorer](https://astexplorer.net/#/gist/1d7b291676388d2d81a8c1f0d48f777b/ac6e92f77315e72f41c605fd5d9afe6cb5a154a8).

We can see that the AST contains a list of statements inside a `BlockStmt`. One of these statements is an `AssignStmt`, and on its RHS (Right Hand Side) is a `CallExpr`, which is our `WithCancel` call.

## Setting up the project

Let’s create a directory structure similar to the following. Feel free to follow something else if you like it better.

```
├── go.mod
├── linters
│   └── ignored_cancel
│       └── ignored_cancel.go
├── main.go
└── testdata
    └── ignored_cancel.go
```

`testdata` contains our test files for verifying our analyzer. `linters` contain our current linter (and any future ones we may write.

To get started, let’s focus on `ignored_cancel.go`. We’ll define our analyzer by declaring a type like the following:

```go
import (
	"golang.org/x/tools/go/analysis"
	"golang.org/x/tools/go/analysis/passes/inspect"
)

var IgnoredCancelAnalyzer = &analysis.Analyzer{
	Name: "ignoredcancel", // name of our analyzer
	Doc: "linter for detecting ignored cancel function returned from context.CancelFunc",
	Run: func(p *analysis.Pass) (interface{}, error) {
		// write our logic here
	}, // the logic for our analyzer
	Requires: []*analysis.Analyzer{inspect.Analyzer}, // declare analyzers that ours is dependent on
}
```

Most of these fields are self explanatory, but let’s look at what `Requires` does. According to the documentation, it takes in analyzers which are treated as dependencies to the one we’re declaring, and should be run before ours runs. Here, we pass in a very interesting analyzer, `inspect.Analyzer`.

What `inspect.Analyzer` does, is that it parses the code into an AST, and provides the result to us in the `Run` function through the `analysis.Pass` argument. Let’s see how we can access it.

```go
func(p *analysis.Pass) (interface{}, error) {
		i, ok := p.ResultOf[inspect.Analyzer].(*inspector.Inspector)
		if !ok {
			return nil, errors.New("analyzer is not of type *inspector.Inspector")
		}
		return nil, nil
}
```

Here, we try and cast the result of the analyzer into `*inspector.Inspector`. This type provides access to the nodes of the AST. If we’re not successful, we return an error.

Now, let’s start traversing the AST. We only want `AssignStmt` nodes, so let’s write a filter for them, and use the convenient `Preorder` function on the `*inspector.Inspector` to perform a Depth-First Search of the AST.

```go
func(p *analysis.Pass) (interface{}, error) {
		i, ok := p.ResultOf[inspect.Analyzer].(*inspector.Inspector)
		if !ok {
			return nil, errors.New("analyzer is not of type *inspector.Inspector")
		}

		// dfs of AST
		filter := []ast.Node{(*ast.AssignStmt)(nil)}
		i.Preorder(filter, func(n ast.Node) {
			// logic
		})

		return nil, nil
	}
```

Now, let’s start writing the core logic for figuring out if the cancelFunc is ignored or not. First, we confirm that the `node` is indeed an `*ast.AssignStmt`, and then inspect it.

```go
i.Preorder(filter, func(n ast.Node) {
	foundIgnoredCancel := false // flag

	// confirm node is indeed an AssignStmt
	node, ok := n.(*ast.AssignStmt)
	if !ok {
		return
	}

	// DFS on the node's children
	ast.Inspect(node, func(n ast.Node) bool {
		// len(RHS) can only be 1 if it's a multi-return function
		// ignore all other cases
		if len(node.Rhs) > 1 {
			return false
		}

		// assert that the RHS is a function call expression
		e, ok := node.Rhs[0].(*ast.CallExpr)
		if !ok {
			return false
		}

		// assert that the function call is a selector expression
		fExpr, ok := e.Fun.(*ast.SelectorExpr)
		if !ok {
			return false
		}

		// assert that the expression in selector is an identifier
		// because it is an import of "context"
		sExpr, ok := fExpr.X.(*ast.Ident)
		if !ok {
			return false
		}

		// if the function signature matches
		if sExpr.Name != "context" || fExpr.Sel.Name != "WithCancel" {
			return false
		}

		// if lhs has more or less variables, something is very wrong
		if len(node.Lhs) != 2 {
			return false
		}

		// assert that the lhs is just an identifier
		lExpr, ok := node.Lhs[1].(*ast.Ident)
		if !ok {
			return false
		}

		if lExpr.Name == "_" {
			foundIgnoredCancel = true
			return false
		}

		return true
	})
	if foundIgnoredCancel {
		p.Reportf(n.Pos(), "found ignored cancelFunc on context.WithCancel")
	}
})
```

Let’s break this down.

1. First we assert that the node is indeed, an AssignStmt.
2. Next, we check if the the RHS has more than 1 expressions. In a multi-function return, there can only be one expression on the RHS. Essentially, we’re checking if `context.WithCancel` was called at all.
3. If the expression on the RHS was a `CallExpr`, we proceed further and check what kind of function call it was.
    
    Go makes a distinction between functions that are part of the language, and functions that are imported. A function that is part of the language will be represented by an `Ident` expr. An imported function will be represented by a `SelectorExpr`, which represents a selection of a particular field of an expression. For example, selection of a field on a struct instance.
    
    In this case, the expression (`X`) will be the package name, and the selector (`Sel`) will be the function name.
    
4. Finally, we check if the expression (package name) is equal to “context” and the selector name (function name) is equal to “WithCancel”. If yes, we check if the 2nd element of the LHS is equal to “_”, which indicates an ignored return value. We set a flag, `foundIgnoredCancel` if this is the case.
5. Finally, we report the position of the lint using `p.Reportf`.

## Running the linter

Let’s wire up the analyzer in `main.go`.

We ensure that the linter will be executed by adding it to the `multichecker.Main` function.

```go
package main

import (
	ignoredcancel "analyzer/linters/ignored_cancel"

	"golang.org/x/tools/go/analysis/multichecker"
)

func main() {
	multichecker.Main(ignoredcancel.IgnoredCancelAnalyzer)
}
```

We can run the analyzer on a sample file containing an ignored cancel statement using the following statement:

```bash
go run main.go -- testdata/ignored_cancel.go

# output
<path>/testdata/ignored_cancel.go:6:2: found ignored cancelFunc on context.WithCancel
exit status 3
```

It works!

# Conclusions and beyond

We demonstrated how simple it is to write your own analyzer for Go. This was a very simple analyzer, but the principles apply nonetheless.

You should note that this implementation is a bit naive, and does not capture all of the possible edge cases. For example, if the “context” package was imported under a different name, our lint would ignore it. The mitigation for this is left as an exercise for the reader :)

Finally, I highly recommend checking out the rest of what the [analysis](https://pkg.go.dev/golang.org/x/tools/go/analysis) package can do. It’s quite powerful and convenient to use.

---

The complete code for this exercise can be found [here](https://github.com/sphericalkat/go-static-analyzer).