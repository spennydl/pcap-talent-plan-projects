## pingcap talent-plan exercises

This repository contains my solutions to the projects presented in the
[pingcap talent-plan repository.](https://github.com/pingcap/talent-plan/tree/master/rust)

The pingcap talent-plan presents a series of projects that iteratively
build up an asynchronous key-value store server in rust.  It leaves out many
implementation details and architecture decisions and instead presents the project
as a spec to be implemented however the programmer sees fit.

This repository contains my implementation of the project.  The code is documented,
and the implementation decisions I made are also documented here.

### Project 1

solution: [cbbb5a82](https://github.com/spennydl/pcap-talent-plan-projects/tree/cbbb5a82dc0fda908404822bb6b660312f635447)

Project 1 gets us started building a simple in-memory key-value store and wrapping
it up in a command line interface.  Check the tests for the spec for the CLI.

Not much to say about this one; the project is a simple CLI wrapper around a map.

### Project 2

in progress

#### Project spec

Project 2 kicks things off with persisting the store to disk using log-structured storage.
Log structured storage is a storage scheme where data is structured as commands and
written to an append-only log.  To illustrate, this may look something like the following:
```
SET key1 value1
SET key2 value2
SET key1 value3
REMOVE key2
SET key5 value5
...
```
The program using this log then keeps an index in memory consisting of a map from keys
to _offsets_ into the log for the _most recent_ record for the key.  An index for the above
log (for example, from keys to line numbers) may look like:
```
key1 -> 2
key5 -> 4
```
This approach helps enable fast updates to and reads from the log: an update is simply
a matter of appending a new record to the log, and the index allows us to jump right into
the file to retrieve the most recent value for a key.  It's also good for concurrency: writers
are always writing to the end while readers are reading from other parts of the log.

There is some overhead, however, as the log will grow forever unless it is cleaned up.  Log
compaction is in the spec for project 2.

#### Implementation

I decided to play with some new concepts that I haven't used much in the past when implementing
this project, specifically memory-mapping files and some binary serialization.

I chose a binary format for serializing log commands.  I didn't feel it was important to have
human-readable logs for this project, and wanted to minimize the amount of space I was using.
I chose [bincode](https://docs.rs/bincode/1.1.4/bincode/) as the serialization format as it
is a tested implementation that seems to have good space efficiency.  If I do need to inspect
the logs at any point in the development process, it should be fairly trivial to write a tool
to dump the logs.

For reading and writing the logs I chose to memory-map the file.  The main impetus here for me
was that I've never had a solid use case for doing this in the past, and I just thought it would
be a fun thing to try.  Given that we will need random access into the file as well as frequently
at the end, I'm not sure yet if this is the best decision performance-wise or if we're going to
run into pagecache issues and faults.  It should help with log compaction, however, which I
anticipate being a mostly linear operation of "dump current index to disk".

This project is still in progress, check back in a bit for updates!
