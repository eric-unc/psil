(define collatz { |n|
	(if (== n 1)
		(put "Reached one!")
		(do
			(print "On " n ".")
			(put)
			(if (== (% n 2) 0)
				(collatz (/ n 2)) ; if even
				(collatz (+ (* n 3) 1))))) ; if odd
})

; 12, 6, 3, 10, 5, 16, 8, 4, 2, 1
(collatz 12)
