package io.github.u2400.wasmtime;

public interface Disposable extends AutoCloseable {
    @Override
    default void close() {
        dispose();
    }

    void dispose();
}
