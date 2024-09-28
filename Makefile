java_run: lib
	javac com/example/fsrs/FSRS.java && RUST_BACKTRACE=full java -Djava.library.path=target/debug com.example.fsrs.FSRS

.PHONY: lib

lib:
	cargo build
