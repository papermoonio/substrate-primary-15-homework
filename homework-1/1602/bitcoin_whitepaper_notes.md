
区块链是什么
区块链是人类有史以来第一次通过技术手段实现了个人财产的神圣不可侵犯  -某个币圈大神说的，具体忘了
区块链是普通民众民反抗各国央行货币发行霸权的一次民主革命  -元一


之前读bitcoin白皮书主要关注点是出块原理，下面是扒的一段矿池先计算不包含coinbase交易tx的部分merkle树，再在获得矿工构建的coinbase交易后合并生成完整merkle树的两个函数，及原理和注释

/**
 * @brief 计算还没有coinbase交易时，为了之后生成merkle根hash
 * coinbase 所需对应的hash对儿的各层所需hash值
 * 有几个值说明计算最终merkle树根就需要几次运算
 * @param input 
 * @return vector<string> 
 */
vector<string> merkle_steps(vector<string> input)
{
	/*
	默克尔树的规则是两两进行hash，如果不够两个则最后一个节点数据额外复制一份作为凑数的最后一个数据
	例如：5个数据 aa  bb  cc dd ee 不够两两配对则 aa与bb 进行 hash cc与dd进行hash  ，ee复制一份添加到末尾与自身进行hash
	hash树的关系结构为：
	aabbcccceeeeeeee    #第五步，最后只剩两个节点后计算出最终的merkle根hash
	aabbccdd eeeeeeee   #第四步，本层两两配对计算出hash
	aabb ccdd eeee eeee #第三步，本层不够双数，凑够双数
	aabb  ccdd  eeee    #第二步，本层两两配对计算出hash
	aa bb cc dd ee ee   #第一步，本层不够双数，凑够双数
	aa bb cc dd ee      #原始数据
	计算顺序由下往上，每层出现不够双数都是由最后一个节点数据填充一份

	由于规定第一笔交易必须是coinbase交易，即上面例子中的aa
	且由于区块头中的nonce只有4字节，随机空间不够，所以在coinbase交易中增加了 8个字节的extren note 随机空间，即该部分是由矿工随机生成的。
	所以coinbase交易必须由矿工生成，这就造成了第一个叶子节点交易即coinbase交易对矿池来说是未知的，merkle根hash也必须由矿工生成。
	这里矿池就有两种方式帮助矿工让其能够计算出merkle根hash，注意矿工只提交区块头相关数据，完整的区块(包括的tx逐笔等数据)是由矿池构建提交的
	第一种是给矿工所有的交易hash即tx，且规定顺序，最后由矿工生成第一个coinbase交易并计算整个merkle树
	（矿工实际上只计算区块头的hash并提交其区块头相关数据，矿工并不提交完整区块，即不提交后面的tx逐笔数据）
	第二种方法:(目前世界上矿池实际采取的方法)
	以??  bb  cc dd ee  ff gg为例 ，虽然开始无法知道?? 即coinbase的hash但是可以先计算后面的可计算部分
	??bbccddeeffgggg
	??bbccdd eeffgggg
	??bb		ccdd		eeff		gggg 
	??		bb		cc		dd		ee		ff		gg
	所以实际只需要给矿工  bb , ccdd , eeffgggg 三个叶子节点的hash值，矿工就能构建出merkle 根，即2的n次方关系，极大减少了所需传递的hash节点数据

	以第一层数据 ?? bb cc dd ee ff  6个元素为例
	首次steps留存L[1] 即bb
	然后从L[2],L[3]开始，两两一组进行hash
	也就是L[0],L[1]，没有参加计算
	结果为  ccdd  与eeff 的hash
	增加第一个"",第二层数据变为  ""  ccdd  eeff  三组
	steps留存L[1] 即ccdd ,此时为steps中存储的是
	bb,ccdd
	由于是奇数组，所以复制最后一个元素到末尾凑成偶数 变为
	""  ccdd  eeff  eeff
	然后从L[2],L[3]开始，两两一组进行hash
	也就是 ""  ccdd 没有参加运算
	结果为  eeffeeff
	增加第一个""，第三层数据变为 ""  eeffeeff
	steps留存L[1] 即eeffeeff ,此时为steps中存储的是
	bb，ccdd，eeffeeff
	此时由于i = 1; i < L.size()/2; 不成立，所以没有操作
	最终结果steps中存储的为
	bb，ccdd，eeffeeff
	也就是这样merkle树           
	""          eeffeeff
	""    ccdd     eeff   eeff
	aa  bb  cc  dd   ee  ff
	之后补充完第一个coinbase  "xx"之后就变成了
	"xx"aabbccddeeffeeff              
	"xx"aabbccdd          eeffeeff
	"xx"aabb      ccdd     eeff    eeff
	"xx"aa    bb   cc   dd   ee   ff
	也就是coinbase 只需要跟 step中存储的数据做依次进行hash运算，则只需3次hash就能够得出merkle根的hash值了
    */
	vector<string> L = input;
	vector<string> steps;
	vector<string> PreL;
	PreL.push_back("");//每层第一个hash为空，为coinbase，及与coinbas进行hash的该层数据保留

	int Ll = L.size();
	while(Ll > 1)
	{
		//存留每层第2个节点的hash值，最后step中保存了几个值，就说明有几层运算
        //最终coinbase交易只需要与steps中的每层的值进行hash即可得出merkle根hash
		steps.push_back(L[1]);

		//如果总个数是奇数(最后一个就配不到对)，则复制一份最后一个元素追加一份copy到最后
		if(Ll % 2)
			L.push_back(L[L.size() - 1]);//L.size()是元素个数，由于下标从0开始，L.size()-1就是最后一个元素，复制一份加到末尾

		vector<string> Ld;//从L[2],L[3]开始，两两一组进行hash后的值，得出高一层的hash值
		for(int i = 1; i < L.size()/2; i++)
		{
			string s = L[i*2] + L[i*2+1];//两组hash相连然后做sha256运算

			char bin[HASHLEN_BIN*2];
			char out[HASHLEN_STR];

			binlify((unsigned char *)bin, s.c_str());
			sha256_double_hash_hex(bin, out, HASHLEN_BIN*2);

			Ld.push_back(out);
		}

		L = PreL;//将L清理成只有一个内容"",代表该层与coinbase交易hash的空缺
		//从参数一迭代器的位置插入，插入内容为从参数2迭代器的位置开始到参数3迭代器的位置结束
        //首次循环至此，第一个元素内容为""，第二个为"ccdd"，第三个为"eeff"
		L.insert(L.end(), Ld.begin(), Ld.end());

		Ll = L.size();//每算一层hash，节点数量减半
	}

	return steps;//返回存储merkle tree中需要与coinbase进行hash的每层的对手hash
}

/**
 * @brief 通过参数2给出的coinbase交易，及参数一给出的各层对手hash计算出merkle树根
 * 
 * @param steps 存储每层所需的节点hash
 * @param f coinbase tx的hash
 * @return string 返回merkle的根hash
 */
string merkle_with_first(vector<string> steps, string f)
{
	vector<string>::const_iterator i;
	for(i = steps.begin(); i != steps.end(); ++i)
	{
		string s = f + *i;

		char bin[HASHLEN_BIN*2];
		char out[HASHLEN_STR];

		binlify((unsigned char *)bin, s.c_str());
		sha256_double_hash_hex(bin, out, HASHLEN_BIN*2);

		f = out;
	}

	return f;
}