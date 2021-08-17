(put (cond
	false "This is obviously false!"
	(== 0 1) "This one too!"
	(!= "happy" "sad") "Okay, you should be able to see this! Hello world!"
	true 5				; Types do not need to match.
	true (/ 1 0)))		; Will not be evaluated because of short-circuiting.
