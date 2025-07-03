# date_shift
Add a second time zone to a list of dates

It's a niche problem but I often have to schedule remote meetings with people in other time zones. So I'll start off by listing some free
times in my own time zone and then add the other person's time zone in parentheses.

This utility will take from stdin:

Wed Jun 11, 9:00am-10:30am PST
Wed Jun 11, 1:00pm-2:30pm PST

and produce:

Wed Jun 11, 9:00am-10:30am PST (12:00pm-1:30pm EST)
Wed Jun 11, 1:00pm-2:30pm PST (4:00pm-5:30pm EST)
