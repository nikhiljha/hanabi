# ws-traces

How to generate a new trace:

1. Open Chrome
2. Open Developer Tools
3. Go to the Network tab
4. Filter by WS
5. Play the game
6. Right click on the WS connection and select "Save as HAR with content"
7. Save the file in this directory
8. Run the following command to clean up the file:

```bash
cat hanab.live.har | jq ".log.entries[4]._webSocketMessages | del(.[].time) | del(.[].opcode)"
```
