(module
  (type (func (param i32 i32)))
  (memory 1 100 shared)
  (func $foo (type 0)
    i32.const 1
    block $foo
      br $foo
    end
    drop
  )
)
