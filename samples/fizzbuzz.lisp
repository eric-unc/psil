(define fizz-buzz {|n end|
	(if (== n end)
		void
		(do
			(if (== (% n 15) 0)
				(put "FizzBuzz")
				(if (== (% n 3) 0)
					(put "Fizz")
					(if (== (% n 5) 0)
						(put "Buzz")
						(put n))))
			(fizz-buzz (+ n 1) end)))})

(fizz-buzz 1 100)
