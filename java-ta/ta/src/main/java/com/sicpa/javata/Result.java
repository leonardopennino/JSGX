package com.sicpa.javata;

import java.util.function.Function;

public sealed abstract class Result<T, E> {
	public static <T> Result<T, ?> wrapping(ThrowingSupplier<T, ?> fn) {
		try {
			T value = fn.get();
			return new Ok<>(value);
		} catch (Throwable e) {
			return new Err<>(e);
		}
	}

	public static <T, E> Result<T, E> Ok(T value) {
		return new Ok<>(value);
	}

	public <U> Result<U, ?> andThen(Function<T, Result<U, ?>> mapper) {
		switch (this) {
			case Ok<T, E> ok -> {
				return mapper.apply(ok.getValue());
			}
			case Err<T, E> err -> {
				return Result.Err(err.getError());
			}
		}
	}

	public <U> Result<U, E> map(Function<T, U> mapper) {
		switch (this) {
			case Ok<T, E> ok -> {
				return Result.Ok(mapper.apply(ok.getValue()));
			}
			case Err<T, E> err -> {
				return Result.Err(err.getError());
			}
		}
	}

	public <U> Result<T, U> mapErr(Function<E, U> mapper) {
		switch (this) {
			case Ok<T, E> ok -> {
				return Result.Ok(ok.getValue());
			}
			case Err<T, E> err -> {
				return Result.Err(mapper.apply(err.getError()));
			}
		}
	}

	public static <T, E> Result<T, E> Err(E error) {
		return new Err<>(error);
	}

	public static final class Ok<T, E> extends Result<T, E> {
		private T value;

		private Ok(T value) {
			this.value = value;
		}

		public T getValue() {
			return value;
		}
	}

	public static final class Err<T, E> extends Result<T, E> {
		private E error;

		private Err(E error) {
			this.error = error;
		}

		public E getError() {
			return error;
		}
	}
}
