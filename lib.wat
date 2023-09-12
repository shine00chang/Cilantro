  ;; (File Descriptor, *iovs, iovs_len, nwritten) -> Returns number of bytes written
  (import "wasi_unstable" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))

  ;; first 20 bytes are reserved for printer
  (memory 1)
  (export "memory" (memory 0))

  ;;@signature $print_int : void (i32)
  (func $print_int (param $x i32)
    (local $i i32)
    (local $j i32)

    ;; Init index
    (local.set $i (i32.const 7))

    ;; Count Size
    (local.set $j (i32.const 1))
    (loop $count 
      ;; Increment Index
      (i32.add (local.get $i) (i32.const 1))
      local.set $i 

      ;; Increase Magnitude
      local.get $j
      i32.const 10
      i32.mul
      local.set $j

      ;; If J <= X, continue
      local.get $j
      local.get $x
      i32.le_u 
      br_if $count
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
      local.set $x
      
      ;; Decrement Index
      (i32.sub
        (local.get $i)
        (i32.const 1)
      )
      local.set $i

      ;; Continue if greater than 0
      (i32.gt_u 
        (local.get $x)
        (i32.const 0)
      )
      br_if $assign 
    )

    ;; Write newline 
    (i32.store8 (local.get $j) (i32.const 10))

    (i32.store (i32.const 0) (i32.const 8))
    (i32.store (i32.const 4) (i32.sub (local.get $j) (i32.const 7)))

    (call $fd_write 
      (i32.const 1)
      (i32.const 0)
      (i32.const 1)
      (i32.const 16)
    )
    drop
  )
