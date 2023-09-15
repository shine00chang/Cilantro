  ;; (File Descriptor, *iovs, iovs_len, nwritten) -> Returns number of bytes written
  (import "wasi_unstable" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))

  ;; first 40 bytes are reserved for printer
  (memory 1)
  (export "memory" (memory 0))

  (func $print32 (param $x i32)
    (local $i i32)
    (local $j i32)

    ;; Init index
    (local.set $i (i32.const 7))

    ;; Count Size
    (local.set $j (i32.const 1))
    (loop $count 
      ;; Increment Index
      (i32.add (local.get $i) (i32.const 1))
      (local.set $i)

      ;; Increase Magnitude
      (i32.mul
        (local.get $j)
        (i32.const 10)
      )
      (local.set $j)

      ;; If J <= X, continue
      (local.get $j)
      (local.get $x)
      (i32.le_u)
      (br_if $count)
    )
    ;; Copy I to J. Add 8 to I
    (local.set $j (i32.add (local.get $i) (i32.const 1)))

    ;; Store to memory
    (loop $assign
      ;; Store Remainder
      (i32.store8 
        (local.get $i)
        (i32.add 
          (i32.const 48)
          (i32.rem_u
            (local.get $x)
            (i32.const 10)
          )
        )
      )
      ;; Set X as Quotient
      (i32.div_u
        (local.get $x) 
        (i32.const 10)
      )
      (local.set $x)
      
      ;; Decrement Index
      (i32.sub
        (local.get $i)
        (i32.const 1)
      )
      (local.set $i)

      ;; Continue if greater than 0
      (i32.gt_u 
        (local.get $x)
        (i32.const 0)
      )
      (br_if $assign)
    )

    ;; Write newline 
    (i32.store8 (local.get $j) (i32.const 10))

    (i32.store (i32.const 0) (i32.const 8))
    (i32.store (i32.const 4) (i32.sub (local.get $j) (i32.const 7)))

    (call $fd_write 
      (i32.const 1)
      (i32.const 0)
      (i32.const 1)
      (i32.const 32)
    )
    (drop)
  )

  ;;@signature $print64 : void (i64)
  (func $print64 (param $x i64)
    (local $i i32)
    (local $j i32)
    (local $k i64)

    ;; Clone into K
    (local.set $k (local.get $x))

    ;; Init index
    (local.set $i (i32.const 7))

    ;; Count Size. Incrementally decrease power of K.
    (loop $count 
      ;; Increment Index
      (i32.add (local.get $i) (i32.const 1))
      (local.set $i)

      ;; Set K as Quotient
      (i64.div_u
        (local.get $k) 
        (i64.const 10)
      )
      (local.set $k)

      ;; Continue if greater than 0
      (i64.gt_u 
        (local.get $k)
        (i64.const 0)
      )
      (br_if $count)
    )
    ;; Copy I to J
    (local.set $j (i32.add (local.get $i) (i32.const 1)))

    ;; Store to memory
    (loop $assign
      ;; Store Remainder
      (i64.store8 
        (local.get $i)
        (i64.add 
          (i64.const 48)
          (i64.rem_u
            (local.get $x)
            (i64.const 10)
          )
        )
      )
      ;; Set X as Quotient
      (i64.div_u
        (local.get $x) 
        (i64.const 10)
      )
      (local.set $x)
      
      ;; Decrement Index
      (i32.sub
        (local.get $i)
        (i32.const 1)
      )
      (local.set $i)

      ;; Continue if greater than 0
      (i64.gt_u 
        (local.get $x)
        (i64.const 0)
      )
      (br_if $assign)
    )

    ;; Write newline 
    (i32.store8 (local.get $j) (i32.const 10))

    ;; Write IOV 
    (i32.store (i32.const 0) (i32.const 8))
    (i32.store (i32.const 4) (i32.sub (local.get $j) (i32.const 7)))

    (call $fd_write 
      (i32.const 1)
      (i32.const 0)
      (i32.const 1)
      (i32.const 32)
    )
    (drop)
  )

  (func $str_len (param $str i64) (result i32)
    (local.get $str)
    (i64.const 32)
    (i64.rotl)
    (i32.wrap_i64)
  )

  (func $str_ptr (param $str i64) (result i32)
    (local.get $str)
    (i32.wrap_i64)
  )

  ;;@signature $print : void (string)
  (func $print (param $str i64) 
    (local $len i32)

    ;; Write len
    (call $str_len (local.get $str))
    (local.set $len)
    
    ;; Copy into memory
    (i32.const 8)
    (call $str_ptr (local.get $str))
    (local.get $len)
    (memory.copy)

    ;; Write iov
    (i32.store (i32.const 0) (i32.const 8))
    (i32.store (i32.const 4) (i32.add (local.get $len) (i32.const 1)))

    (call $fd_write 
      (i32.const 1)
      (i32.const 0)
      (i32.const 1)
      (i32.const 8)
    )
    (drop)
  )
