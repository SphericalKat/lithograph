# Tools of the Trade: Declarative DNS with dnscontrol

![Header](https://dev-to-uploads.s3.amazonaws.com/uploads/articles/zdrj2e0lb1079644mh9k.png)

## 💡 A bit of context
I recently moved my [site](https://sphericalk.at)'s hosting from DigitalOcean to [fly.io](https://fly.io). That's when I realized that I've been changing DNS records manually for years; go to the Cloudflare dashboard, change records, rinse-repeat...

As a programmer and self-professed automation fanatic, this rubbed me the wrong way, so I set out to do something about it. Enter DNSControl.


## 🤔 Hold up, what's that?
[DNSControl](https://stackexchange.github.io/dnscontrol/) is an **opinionated** platform for seamlessly managing your DNS configuration across any number of DNS hosts, both in the cloud or in your own infrastructure.

It was created by StackOverflow to manage their own domains, and was subsequently open-sourced for the rest of us (Thanks StackOverflow!).

It lets you _declare_ your DNS recordings using a JavaScript-ish DSL (domain specific language); for example:

![Example of a DNS configuration using dnscontrol](https://dev-to-uploads.s3.amazonaws.com/uploads/articles/ofqaevh7g2vd0d1xidrp.png)

## Okay, why should I switch?
Switching to a new tool can be scary, but here's some reasons that make DNScontrol unrivaled (in my opinion):

- Supports 10+ DNS Providers including BIND, AWS Route 53, Google DNS, Cloudflare, and name.com
- Enable/disable Cloudflare proxying (the "orange cloud" button) directly from your DNSControl files.
- Apply CI/CD principles to DNS: Unit-tests, system-tests, automated deployment.
- Super extensible! Plug-in architecture makes adding new DNS providers and Registrars easy!
- Eliminate vendor lock-in. Switch DNS providers easily, any time, with full fidelity.
- All the benefits of Git (or any VCS) for your DNS zone data. View history. Accept PRs.

And much more!

## 🤨 Wait! I heard opinionated?
That's right. DNSControl has opinions about how to manage DNS. However, that isn't a bad thing. Let's go over their opinions and see which ones really affect us.

1. DNS should be treated like code:
Code is written in a high-level language, version controlled, commented, tested, and reviewed by a third party… and all of that happens before it goes into production.
2. Non-experts should be able to safely make DNS changes.
3. All DNS is lowercase for languages that have such a concept.
4. Users should state what they want, and DNSControl should do the rest.
5. If it is ambiguous in DNS, it is forbidden in DNSControl.
6. Hostnames don’t have underscores.

These make total sense. To read more about the opinions and the reasons behind them, check out the [documentation](https://stackexchange.github.io/dnscontrol/opinions).

## ✅ Okay, I'm convinced. How do I begin?
Glad to hear it! To get started, first install DNSControl.
```
go install github.com/StackExchange/dnscontrol/v3@latest
```

Now, create a file called `dnsconfig.js`. This will house all the records for your site(s). Populate it with the starter boilerplate:
```js
// Providers
var REG_NONE = NewRegistrar('none');    // No registrar.
var DNS_BIND = NewDnsProvider('bind');  // ISC BIND.

// Domains
D('example.com', REG_NONE, DnsProvider(DNS_BIND),
    A('@', '1.2.3.4')
);
```

Next, create a file called `creds.json` for storing provider configurations (API tokens and other account information). For example, to use both `name.com` and `Cloudflare`, you would have:
```json
{
  "cloudflare": {
    "TYPE": "CLOUDFLAREAPI",
    "accountid": "your-cloudflare-account-id",
    "apitoken": "your-cloudflare-api-token"
  },
  "namecom": {
    "TYPE": "NAMEDOTCOM",
    "apikey": "key",
    "apiuser": "username"
  },
  "none": { "TYPE": "NONE" } // the no-op provider
}
```

For a complete list of providers and how to use them, check out the [provider docs](https://stackexchange.github.io/dnscontrol/provider-list).

> **Warning**: Remember to add `creds.json` to your `.gitignore` _before_ committing and pushing your configuration.

Now, run `dnscontrol preview` and make sure that it completes with no errors. The preview command is the “dry run” mode that shows what changes need to be made and never makes any actual changes. It will use APIs if needed to find out what DNS entries currently exist.

The results should look something like this:
```
dnscontrol preview
Initialized 1 registrars and 1 dns service providers.
******************** Domain: example.com
----- Getting nameservers from: bind
----- DNS Provider: bind... 1 correction
#1: GENERATE_ZONEFILE: example.com
 (2 records)

----- Registrar: none
Done. 1 corrections.
```

Next, run `dnscontrol push` to actually make the changes. In this case, the change will be to create a zone file where one didn’t previously exist.

```
dnscontrol push
Initialized 1 registrars and 1 dns service providers.
******************** Domain: example.com
----- Getting nameservers from: bind
----- DNS Provider: bind... 1 correction
#1: GENERATE_ZONEFILE: example.com
 (2 records)

CREATING ZONEFILE: zones/example.com.zone
SUCCESS!
----- Registrar: none
Done. 1 corrections.
```

## ⚙️ I already have many DNS records set up. How do I import them?

This was a problem I faced myself, and this is how you import your existing DNS configurations.

Most DNS Service Providers have an 'export to zonefile' feature. (If yours doesn't, consider moving to Cloudflare; It's free, they'll import all your records during setup, and will let you export a zonefile)

Place this zonefile inside a directory called `zones` in the directory where your `dnsconfig.js` resides. Rename the zonefile to `<domain>.zone`. Your directory structure should now look something like this:
```
.
├── creds.json
├── dnsconfig.js
└── zones
    └── example.com.zone
```

Next, run the following command:
```
dnscontrol get-zones --format=js --out=draft.js bind BIND example.com
```

Now you can copy the contents of `draft.js` to `dnsconfig.js` and modify as needed.

## 🚀 Lastly, some advice for running in production
- Store the configuration files in Git.
- Encrypt or completely omit the `creds.json` file before storing it in Git. Do NOT store API keys or other credentials without encrypting them.
Use a CI/CD tool like Github Actions/Jenkins/CircleCI/etc. to automatically push DNS changes.
Join the DNSControl community. File [issues and PRs](https://github.com/StackExchange/dnscontrol).

I hope this tool as just as useful to you as it is to me. Stay tuned for further posts about useful tooling.