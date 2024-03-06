use std::collections::LinkedList;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// Структура `HashMap`, реализующая хэш-таблицу.
pub struct HashMap<K, V> {
    buckets: Vec<LinkedList<(K, V)>>, // Вектор связных списков для разрешения коллизий
    items_count: usize, // Количество элементов в хэш-таблице
    load_factor: f64, // Коэффициент загрузки для определения момента рехеширования
}

impl<K: Eq + Hash, V> HashMap<K, V> {
    /// Создает новую хэш-таблицу.
    ///
    /// # Примеры
    ///
    /// ```
    /// use your_crate::HashMap;
    /// let mut map: HashMap<i32, String> = HashMap::new();
    /// ```
    pub fn new() -> Self {
        let initial_capacity: usize = 1;
        HashMap {
            buckets: Vec::with_capacity(initial_capacity),
            items_count: 0,
            load_factor: 2.0,
        }.initialize_buckets(initial_capacity)
    }

    /// Инициализирует внутренние корзины хэш-таблицы.
    fn initialize_buckets(mut self, capacity: usize) -> Self {
        for _ in 0..capacity {
            self.buckets.push(LinkedList::new());
        }
        self
    }

    /// Вставляет ключ и значение в хэш-таблицу.
    ///
    /// # Примеры
    ///
    /// ```
    /// use your_crate::HashMap;
    /// let mut map = HashMap::new();
    /// map.insert(1, "один");
    /// ```
    pub fn insert(&mut self, key: K, value: V) {
        let index: usize = self.get_index(&key);
        if self.buckets.is_empty() || index >= self.buckets.len() {
            self.resize();
        }
        if !self.buckets[index].iter().any(|(k, _)| *k == key) {
            self.buckets[index].push_back((key, value));
            self.items_count += 1;
            if self.current_load_factor() > self.load_factor {
                self.rehash(self.buckets.len() * 2 + 1);
            }
        }
    }

    /// Удаляет значение по указанному ключу из хэш-таблицы.
    ///
    /// # Примеры
    ///
    /// ```
    /// use your_crate::HashMap;
    /// let mut map = HashMap::new();
    /// map.insert(1, "один");
    /// map.remove(&1);
    /// ```
    pub fn remove(&mut self, key: &K) {
        let index: usize = self.get_index(key);
        let bucket: &mut LinkedList<(K, V)> = &mut self.buckets[index];
        let mut new_bucket: LinkedList<(K, V)> = LinkedList::new();
        while let Some((node_key, node_value)) = bucket.pop_front() {
            if &node_key != key {
                new_bucket.push_back((node_key, node_value));
            } else {
                self.items_count -= 1;
            }
        }
        *bucket = new_bucket;
    }

    /// Возвращает ссылку на значение, связанное с указанным ключом.
    ///
    /// # Примеры
    ///
    /// ```
    /// use your_crate::HashMap;
    /// let mut map = HashMap::new();
    /// map.insert(1, "один");
    /// assert_eq!(map.get(&1), Some(&"один"));
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        let index: usize = self.get_index(key);
        self.buckets[index].iter().find_map(|(k, v)| if k == key { Some(v) } else { None })
    }

    /// Вычисляет текущий коэффициент загрузки хэш-таблицы.
    /// Коэффициент загрузки определяется как отношение количества элементов к количеству корзин.
    ///
    /// Возвращает текущий коэффициент загрузки в виде числа с плавающей точкой.
    fn current_load_factor(&self) -> f64 {
        self.items_count as f64 / self.buckets.len() as f64
    }

    /// Выполняет рехеширование хэш-таблицы, изменяя размер внутреннего вектора корзин.
    /// Этот метод вызывается, когда коэффициент загрузки превышает установленный порог.
    fn rehash(&mut self, new_size: usize) {
        if new_size <= self.buckets.len() {
            return;
        }

        let mut temp: Vec<(K, V)> = Vec::new();

        for bucket in self.buckets.drain(..) {
            for (key, value) in bucket {
                temp.push((key, value));
            }
        }

        self.buckets = Vec::with_capacity(new_size);
        for _ in 0..new_size {
            self.buckets.push(LinkedList::new());
        }

        for (key, value) in temp {
            let index = self.get_index_for_key(&key, new_size);
            self.buckets[index].push_back((key, value));
        }
    }

    /// Увеличивает размер хэш-таблицы, удваивая количество корзин.
    /// Этот метод вызывается, когда требуется расширить хэш-таблицу для улучшения производительности или вместимости.
    fn resize(&mut self) {
        let new_size: usize = match self.buckets.len() {
            0 => 1,
            n => n * 2,
        };
        self.rehash(new_size);
    }

    /// Вычисляет индекс корзины для заданного ключа.
    /// Использует хэш-функцию ключа и размер хэш-таблицы для определения индекса.
    ///
    /// Возвращает индекс вектора корзин, в который должен быть помещен элемент.
    fn get_index(&self, key: &K) -> usize {
        self.get_index_for_key(key, self.buckets.len())
    }

    /// Вычисляет индекс корзины для заданного ключа и указанного размера хэш-таблицы.
    /// Этот метод используется внутренне при рехешировании для определения новых индексов элементов.
    ///
    /// Возвращает индекс вектора корзин, в который должен быть помещен элемент.
    fn get_index_for_key(&self, key: &K, size: usize) -> usize {
        let mut hasher: DefaultHasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % size
    }
}
