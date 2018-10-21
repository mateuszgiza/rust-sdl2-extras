use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

// Generic trait to Load any Resource Kind
pub trait ResourceLoader<'l, TResult> {
    type Args: ?Sized;
    fn load(&'l self, data: &Self::Args) -> Result<TResult, String>;
}

// Generic struct to cache any resource loaded by a ResourceLoader
pub struct ResourceManager<'l, TKey, TResult, TLoader>
where
    TKey: Hash + Eq,
    TLoader: 'l + ResourceLoader<'l, TResult>,
{
    loader: &'l TLoader,
    cache: HashMap<TKey, Rc<TResult>>,
}

impl<'l, TKey, TResult, TLoader> ResourceManager<'l, TKey, TResult, TLoader>
where
    TKey: Hash + Eq,
    TLoader: ResourceLoader<'l, TResult>,
{
    pub fn new(loader: &'l TLoader) -> Self {
        ResourceManager {
            cache: HashMap::new(),
            loader: loader,
        }
    }

    // Generics magic to allow a HashMap to use String as a key
    // while allowing it to use &str for gets
    pub fn load<D>(&mut self, details: &D) -> Result<Rc<TResult>, String>
    where
        TLoader: ResourceLoader<'l, TResult, Args = D>,
        D: Eq + Hash + ?Sized,
        TKey: Borrow<D> + for<'a> From<&'a D>,
    {
        self.cache.get(details).cloned().map_or_else(
            || {
                let resource = Rc::new(self.loader.load(details)?);
                self.cache.insert(details.into(), resource.clone());
                Ok(resource)
            },
            Ok,
        )
    }
}