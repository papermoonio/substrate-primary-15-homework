export default {
    /**
     * @category Practical
     * @name localCache.setItem localStorage存值
     * @function localCache.setItem(key,value)
     * @description
     * 在localStorage中存值.
     * @param {string} key 要存的localStorage的key值
     * @param {*} value 要存的localStorage的value值
     * @example
     * localCache.setItem('localName', 'localValue')
     */
    setItem(key, value) {
      localStorage.setItem(key, JSON.stringify(value));
    },
    /**
     * @category Practical
     * @name localCache.getItem localStorage取值
     * @function localCache.getItem(key)
     * @description
     * 在localStorage中取值.
     * @param {string} key 要取的localStorage的key值
     * @returns {string} 从localStorage中取出的值
     * @example
     * localCache.getItem('localName')
     * // => 'localValue'
     */
    getItem(key) {
      try {
        const result = localStorage.getItem(key);
        return JSON.parse(result);
      } catch (err) {
        return;
      }
    },
    /**
     * @category Practical
     * @name localCache.removeItem localStorage删除数据
     * @function localCache.removeItem(key)
     * @description
     * 在localStorage中删除数据.
     * @param {string} key 要删除的localStorage的key值
     * @example
     * localCache.removeItem('localName')
     */
    removeItem(key) {
      localStorage.removeItem(key);
    },
  };