# Using Cloudflare Workers and GitHub Actions to deploy statically generated sites

<p class="m-0">Deploying performant websites at massive scale has never been easier (or cheaper)</p>

---

## All hail the JAMStack

Sites built on the JAMStack are all the rage these days; they’ve become a popular alternative to full-stack frameworks and server-side rendered web applications. They’re lightweight, performant, and are extremely easy to deploy on platforms like [Vercel](https://vercel.com) or [Netlify](https://netlify.com). With that in mind, let’s take a look at how we can step up our deployment game and reduce load times to a minimum.

### Netlify is great... but?

Enter Cloudflare Workers. Workers are a serverless platform that allow you to deploy code in various languages like JavaScript, Rust, Python, Kotlin etc. A huge advantage that they possess over other serverless platforms such as AWS Lambda is that workers automatically deploy your code across the globe, thanks to Cloudflare’s massive CDN.

### Serverless code is great, but what about content?

Workers also recently released a KV store, that can be used to store static content such as CSS or JS chunks. This makes workers ideal for deploying a performant site at massive scale. -->

## That's great! How do we use it?

Whoa there, hold yer horses. Let’s understand what a worker does exactly before jumping into it.
A worker is a piece of code that’s executed when a particular route on a website proxied by Cloudflare is accessed, before the request reaches Cloudflare’s cache.
The following is a super simple worker, which just responds with a `Hello World!`.

```js
addEventListener('fetch', event => {
  event.respondWith(new Response('Hello World!'))
})
```

### Wrangling your workers with wrangler

Cloudflare has a great tool to configure workers called wrangler. Install it globally using:

```bash
# With NPM
npm i -g @cloudflare/wrangler
# With yarn
yarn global add @cloudflare/wrangler
```

Or just add it to your `devDependencies`.

Now we need to authorize wrangler to create and edit workers. Run `wrangler login` to automatically add an API key to your local wrangler config file.

### Configuring a domain

Every site needs a domain, and ours is no different. First, you need your domain added to the Cloudflare dashboard. I already have sphericalkat.dev added to mine.

In the DNS section, add a record of type A, with whatever subdomain you wish that points to any unreachable IP address like 192.2.0.1. This isn’t strictly necessary, but it’s recommended since our worker will intercept all requests and the IP will never resolve.

![](https://miro.medium.com/max/1400/1*LPAwz7akuOu7FzdeiDycQQ.png)
