# weird.one
Local-first linkspage generator

## Usage
You'll need Perseus and SurrealDB.  
Start the SurrealDB instance. 
```
surreal start -u root -p root --log debug "file://weirddb"
```


Start Perseus
```
perseus serve -w
```


Go to `localhost:8080`

### Introduction

We're building a linksapp (linktree, bio.link et.al.) for developers. It will interface with the GitHub network and API.

In our more expansve vision of this app we see it as the beginning of a new kind of minimalistic social network. One that isn’t trying to push an ads-driven feed on its users, but is rather all about connection around shared purpose.

https://blog.jim-nielsen.com/2022/other-peoples-websites/

*Weird is a..*
- [Network of shared purpose](https://github.com/commune-org/weird/issues/9)
- [Progressively advanced CMS](https://github.com/commune-org/weird/issues/5)
- [Web identity aggregator](https://github.com/commune-org/weird/issues/1)

### Origins

> *@erlend-sh said:*
> 
> Right now I use GitHub as my go-to profile page: https://github.com/erlend-sh
> 
> It works quite well, since practically all of my ongoing projects, professional and otherwise, are taking place on GitHub. It still leaves a lot to be desired however.
>
> A growing number of people in gamedev actually run personal Discord servers. It seems a bit like something an egomaniac would do (and it can be), but really it makes perfect sense for anyone who want to be showing how to do something to multiple people.
> 
> I have this erlend.sh domain that I’m not using. I wanna transcribe most of my GitHub personal page onto there, so I finally have my ultimate calling-card and place to call my own on the interwebs. Essentially an open source alternative to <https://linktr.ee>.
> 
> Perseus can do this very effectively, and with added super powers to grab your personal content from cloud APIs.
> 
> I’ve done so many projects over the years, and for each and every one of them there’s always someone out there with the interest and capability to help me solve some problem, usually because they've already been solving that problem for their own project. But it’s unnecessarily difficult to find these people, and for people to find me.
> 
> I wanna make it much simpler for people on the internet to go ‘I’m interested in working on x/y/z’ and be connected with other people who share that interest, especially among open source practitioners.
> 
>> This sounds super cool, I myself have wished for a platform like this. There are numerous community platforms/forums but haven't seen something focused on common interest network. So aside from the links page, there would be say tags we can set on our profiles that gets on the radar of people to discover people looking for that interest or people with similar interest?
> 
> Yep, exactly. And there would be a simple directory & search based on people’s public profiles. For my @fishfolks project for instance, I wanna know about people who are into ‘open source’, ‘gamedev’ and ‘rust’. I’ve searched manually in this way on GitHub many times, and connected with dozens of collaborators that way.
> 
> I’ll for example see that someone already worked on a 2D pixel platformer game in Rust already, so I just get in touch and say ‘hey, seems like we might have a shared interest! Wanna talk?’. There’s a rather incredible amount of people out there working on the same kind of stuff without ever getting in touch with *the others*.

#### Prior art

- https://github.com/commune-org/linksapp-fresh (our own prototype in Fresh/Deno)
- https://github.com/Neutron-Creative/Singlelink
- https://github.com/sethcottle/littlelink
  - https://github.com/JulianPrieber/littlelink-custom
  - https://littlelink-custom.com/blog/ai-generated-theme/
- https://codeberg.org/keyoxide/
