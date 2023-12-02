# advent2023
Advent of code until i fall asleep

### Script to download input explanation in case reddit croaks

```
curl https://adventofcode.com/2018/day/DAY/input --cookie "session=SESSION"
```

You can find SESSION by using Chrome tools. Go to https://adventofcode.com/2018/day/3/input, right-click, inspect, tab over to network, click refresh, click input, click cookies, and grab the value for session. (Is there a simpler way?)

Apparently it needs to be said: don't run this in a loop. It will hurt the website. Once is enough.

