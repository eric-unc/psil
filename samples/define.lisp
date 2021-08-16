(define pi 3)

; not so accurate lol but a good demo
(define sin {|x| x})
(define cos {|x| (- 1 x)})

(put "What's the sin of 0?")
(print (sin 0) "!")
(put)

(put)
(put "What's the cos of 0?")
(print (cos 0) "!")
(put)

(put "How about the cos/sin of pi?!?")
(print "Cos: " (cos pi) ", sin: " (sin pi) "!")
(put)

(put)
(put "(Yeah that's totally correct!)")
