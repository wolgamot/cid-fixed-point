# cid-fixed-point
Program to look for a self describing ATProto record

## Description
Generates a random ATProto record based on the one given and checks the CID, then places that CID in the json value and checks for the new CID, then checks if those two match. Bluesky already allows self referencing records but only because it doesn't check the CID to see if it matches, so if this exists it would be a 100% valid and protocol aproved way to make a dumb post, that would also just be a jumble of letters. There may also be security considerations for fixed points in hashes? but I don't know about that so don't quote me.

I wrote this in like a day and have no idea if it works or if it does what it says on the tin but it compiles.

## Dependencies
Didn't feel like fucking with libraries so you only need `ipfs` which is run as a command by the code. 

## Usage
1. Make a Bluesky post
2. Get the record (you can use [PDSls](https://github.com/notjuliet/pdsls))
3. Make a file containing the record on `src/record.json`
4. Build and run with cargo
5. Good luck!

Fuck off
