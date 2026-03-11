from std.memory import UnsafePointer

@export
fn mojo_process_block(address: Int, size: Int):
    # No Mojo 0.2X, a sintaxe correta para reconstruir ponteiros externos mutáveis é:
    # UnsafePointer[Type, MutAnyOrigin](unsafe_from_address=...)
    var data = UnsafePointer[Float32, MutAnyOrigin](unsafe_from_address=address)
    
    for var i in range(size):
        data[i] = data[i] * 0.5

@export
fn mojo_init(sample_rate: Float64):
    pass