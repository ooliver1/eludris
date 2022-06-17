# Eludris

This markdown file is dedicated to explaining and organizing how the features
and components of Eludris will work.

If a line starts with [STC] (Subject To Change) it hasn't been fully decided on
and can be altered or straight up removed at any given point in time.

This file is changed every while with new changes and additions appearing.

## Overview

The goal is to provide a uniquely fresh but not totally new experience focused
around programming but also any other topic / interest whilst being secure, free 
(as in monetary value), free (as in freedom), privacy respecting, open source
and decentralized without inconveniencing users who may not have anything to do
with the aforementioned topics.

- You can create communities.

Communities can be an either a messaging community, a post community or both.

Messaging communities work like how a discord server would.

Post communities work like how a reddit subreddit would.

Communities can be either public or private.

Communities can get manually reviewed by a staff member to get verified if requested
by the community's owner.

Verified communities can claim a namespace getting their own url invite and are indexed
in a list to be easily discoverable.

Verified communities have more restrictions upon them however
(like no e2ee, stricter moderation and so on).

A community can get unverified at any point in time if they break the Eludris
EULA, TOS or instance rules, that however can be appealed and unjustified
unverification will not happen.

- Accounts are unique.

Much like reddit and twitter only one person can have a specific account name,
HOWEVER that limitation is broken upon multiple instances, refer to the
[federation](#federation) section.

You can follow people or send them a friend request.

You also gain some sort of karma like point creativity called social credits
depending on how well you are generally received, how much you interact with 
people and so on.

- Bots done *right*

[STC] Bots will be user accounts (like you'd see on reddit or twitter) which
would mean that they wouldn't have any api limitation because if you can do
something malicious with a bot account you can do it with a user account so I
think we should focus on making all types of accounts relatively unexploitable.

Discord style application commands will be available, however unlike discord
they will not be forced upon people and will have more utilities.

Also from Discord, buttons and more message components will be available to
give bot developers more room and tools to make cool stuff.

## Tokens

About tokens, other than having your signup email and password for every request
you make to `/auth` you get a session token, you ideally should have one session
token for every browser / app you use with your Eludris instance, that makes it
possible to invalidate them easily if they ever get compromised.

Changing your password automatically invalidates all your tokens.

Upon connecting to the gateway (Pandemonium) and sending an auth request, you
get a Pandemonium token which unlike your other tokens or password aren't
hashed to reduces the load on the server for every request you send, you can
still use your session token but if you're going to connect to Pandemonium,
it's recommended to use the provided Pandemonium token.

Pandemonium tokens are stored temporarily in the cache and are deleted after the
client disconnects meaning you can't reuse a Pandemonium token after you disconnect.

## End To End Encryption (e2ee)

End to End encryption will be available in private communities, group encrypted
group chats and DMs between friends.

### E2EE Implementation

For starters, all users have a personal public key and private key.

Events with encrypted data (message, post, etc) have an extra field in their
payload, the `pubkey` field which contains the public key the message's content
was encrypted with so that the corresponding private key would be fetched from the
user's device's local storage or requested if it doesn't exist or the
current one is invalid.

#### DMs

This one is quite simple, upon a friend request getting accepted and two users
becoming friends, the user who accepted the friend request sends a payload with
a public key and a private key for the DM, both encrypted using the other user's
public key.

After that all messages sent in this DM is encrypted using the DM's public key and
are decrypted with their private key which are now stores on both user's devices.

A user can also request they get a new key from the other end which will entirely
scrap the old pair of keys and generate new ones in case the old ones get
compromised or they didn't reach the other user for any reason.

#### Group DMs

Group DMs can be encrypted too, they work in a simmilar fashion, the host sends the
room's public and private keys to all the starting participants on room creation
encrypted with their public keys.

Any user can request they get the private and public keys of the room again in which
any available user would just encrypt it using the request's user's public key and
send it back.

Upon a new user joining the GroupDM they could just request the room's keys.

The room's key's can also be re-generated by the DM's host.

#### Private Communities

Privtate communities work simmilarly to how GroupDMs work with the addition that
the posts may also be encrypted but follow the same foundations.

## Federation

Eludris will be federated, meaning anyone can host their own instance and they
can communicate with other instances so that any user on one instance can
interact with others on any other instance.

### Side note about federation

Making your own implementation in the language you prefer is actually encouraged,
same with forking this one and adding new stuff to it, just make sure to give your
new features an id and name so that clients that work with multiple instances can
use your features & not break if you made substantial changes, incidentally everything
included in this implementation is called `base` and has an id of `0`, all other
implementations should at least have them.

Features are acquired by sending a `GET` request to an instance's `/` route besides
the other elements of the `info` payload.

### [STC] Federation Implementation

All routes where other instances can request / submit data will have an
additional `/external` route (like `/external/this/channels/:channelid/`).

For info about how IDs are created read [this](#ids).

`/external` routes will follow specific rules, these being:

A new instance (one the home instance has never seen before) will have to
first send an `identify` payload, this payload is simple as it just provides
a shared **private** key that both instances can indentify each other with (in
case an instance's domain get's compromised) and the instance's id.

`/external` routes will take both http requests payloads and Pandemonium payloads
in the form of http requests (let's say an instance A has a community with a
channel that has user's from other instances, one of which is B when a user from
instance B sends a message to `B's domain/external/A's ID/channels/:channelid/
messages` B will send the rest api message payload to `A's domain/external/
A's ID/channels/:channelid/message`, and when a user from instance A send's a
message the opposite will happen with A sending a request to B's external route).

I'm sure this implementation has some edge cases which may cause some issues but
I'm hoping to iron them out and doccument them here once I encounter them.

## Miscellaneous info

### IDs

A Eludris ID is a 32 bit (4 byte) number, structured like so.

```
 12345678  12345678  12345678  12345678  12345678  12345678  12345678  12345678
 TTTTTTTT  TTTTTTTT  TTTTTTTT  TTTTTTTT  SSSSSSSS  SSSSSSSS  RRRRRRRR  RRRROOOO
╰──────────────────────────────────────╯╰──────────────────────────────────╯╰─╯
                   │                                      │                │
                   │                                      │ 4 bit (0.5 byte) Overflow counter.
                   │                      28 bit (2.5) byte uniqueness check.
      32 bit (4 byte) Unix Timestamp.
```

T: A Unix timestamp with a custom epoch
( 1,650,000,000 + (32 bit limit * overflow counter value))

S: A sequence number that's increased everytime an ID is generated then reset
once over 2 bytes.

R: A 12 bit random number.

O: The overflow counter, increases by one once the unix timestamp passes the 16
bit integer limit.

### Redis

Eludris uses a non persistent redis instance to store data that should be really
fast to fetch and is ephemeral

like the Pandemonium tokens and the ratelimiting data.

Here's the structure of the two redis keys:

- token:\<user-id>
- ratelimit:\<user-id>:\<method>:\<route>

### Stack

- [Rust](https://rust-lang.org) Programming Language.
  - [rocket](https://rocket.rs) Rest Api Framework.
  - [tokio-tungstenite](https://github.com/snapview/tokio-tungstenite)
    Pandemonium (gateway)
  Handler.
  - [sqlx](https://github.com/launchbadge/sqlx) SQL Query handler.

  Note:

     > We were planning to use async-diesel but since diesel doesn't play well
     with MariaDB (mainly the diesel-cli) we switched to sqlx and the database will
     be mapped manually.
  - [redis-rs](https://github.com/mitsuhiko/redis-rs) Crate for interfacing with
  redis.

- [MariaDB](https://mariadb.org) Database.
- [Redis](https://redis.io) Cache for [Pandemonium tokens](#tokens) & ratelimit info.

### Internal names

Some of the Eludris components have names that are used internally by the
Eludris dev team or are referenced directly in the source code.

Here are some of these names:

- Das Ding: The Eludris logo.
- Thang: The Eludris mascot.
- Pandemonium: The Eludris Websocket based gatway.
- Carnage: The official Eludris frontend.

## API Spec

### Payloads

### Routes
