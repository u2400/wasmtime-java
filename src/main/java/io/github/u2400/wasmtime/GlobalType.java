package io.github.u2400.wasmtime;

import lombok.AllArgsConstructor;
import lombok.Value;

@Value
@AllArgsConstructor
public class GlobalType {
    Val.Type content;
    Mutability mutability;
}
