from std.memory import UnsafePointer

# ─── Funções @export (Interface C para o Rust via FFI) ────────────────────────

@export
fn mojo_init(sample_rate: Float64):
    """Inicializa o processador com a taxa de amostragem do host."""
    pass

@export
fn mojo_process_block(address: Int, size: Int, drive: Float32, output_gain: Float32):
    """Processa um bloco de áudio in-place — Zero-Copy FFI.

    Recebe o ENDEREÇO de memória do buffer f32 do Rust como Int.
    Reconstrói o ponteiro mutável internamente (Address Bypass Pattern).
    Aplica saturação suave (tanh polinomial) sample-a-sample.

    Args:
        address:     Endereço de memória do buffer *mut f32.
        size:        Número de amostras no buffer.
        drive:       Ganho de entrada (pre-gain).
        output_gain: Ganho de saída (post-gain).
    """
    var data = UnsafePointer[Float32, MutAnyOrigin](unsafe_from_address=address)

    for var i in range(size):
        var x = data[i] * drive
        # Clamp para evitar overflow na aproximação de tanh
        if x > 4.0:
            x = 4.0
        elif x < -4.0:
            x = -4.0
        # Approximacao polinomial de tanh: x*(27+x^2)/(27+9*x^2)
        var x2 = x * x
        var saturated = x * (27.0 + x2) / (27.0 + 9.0 * x2)
        data[i] = saturated * output_gain