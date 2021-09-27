(put "Hello, World!")

(put) ; extra new line

; approximation for x near 0
(define sin {|x| x})

(put "What's the sin of 0?")

(define num (sin 0))

(if (== num 0)
	(put "Yay, sin(0) is 0!")
	(put "How can sin(0) not be 0???"))
