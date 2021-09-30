(define x (str-cat "Hello" " " "world" "! " (/ 20 2) " " str-cat))
(put x)

(put (str-len x))

(put (str-contains? x "world")) ; true
(put (str-contains? x "world4")) ; false

(put (str-trunc "hellohello" 3)) ; hel

(put (str-insert "bar" 0 "foo")) ; foobar
