; end is exclusive
(define fizz-buzz {|n end|
	(if (== n end)
		void		; stop if at the end
		(do			; else, print whatever and increment
			(cond
                (== (% n 15) 0) (put "FizzBuzz")
                (== (% n 3) 0) (put "Fizz")
                (== (% n 5) 0) (put "Buzz")
                true (put n))
            (fizz-buzz (+ n 1) end)))})

(fizz-buzz 1 100)
