
trait Transformer<T, R, Env> {
    fn transform(&self, env: Env, x: T) -> R;
}