# Long Polling with a timer

## General Background

This is a runtime implementation that long polls a remote server, and iterates either: 
- if there is an update to the remote server
- if an interval has completed(in this case 10 seconds)

## Project Description

/long-poll-server - The server implementation that has one GET /messages and one POST /messages, both managing a long-polled entity.   
/future-poll - Polling using futures as opposed to a stream-like interface using channels.   
/stream-poll - Polling using a stream-like interface, implemented with channels.   

## Difference between future-poll and stream-poll

Timeline of the two: 

Futures impl:
---------0---------0---------0---------0-----X----------0----------0----------0----------0----X----------0----------0----------0

Streams impl:
---------0---------0-----X---0---------0---------0----------0----------0-----X----0----------0---------X0----------0----------0----------0

Where:   
- '0' denotes the interval iteration
- 'X' denotes the long poll iteration
- '-' denotes a time unit. 

The timing signatures are slightly different - the futures impl will "short-circuit" the interval, and if the long poll returns a new value, it will wait a full interval 
cycle to iterate another value. The streams impl will not do this kind of behaviour, but simply iterate on it's own time. This causes intances where the iteration
may have happened in a very short time-period. 

