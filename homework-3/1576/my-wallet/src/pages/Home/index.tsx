import { trim } from '@/utils/format';
import { PageContainer } from '@ant-design/pro-components';
import { useModel } from '@umijs/max';
import { Button, Card, Col, Row, Input, message, Space, Form, InputNumber } from 'antd';
import styles from './index.less';

const HomePage: React.FC = () => {
  const { 
    mnemonic, setMnemonic, createAccount, address, setAddress,
    balances, getBalances, loading, to, setTo,
    transfer, setAmount, claim, setClaim, createClaim
  } = useModel('polkadot');
  console.log(useModel('polkadot'));

  const onMnemonicChange = (e: any) => {
    setMnemonic(e.target.value);
  };

  const onClaimChange = (e: any) => {
    setClaim(e.target.value);
  }

  const onPasteMne = (e: any) => {
    const text = handlePaste(e);
    // 处理粘贴内容
    setMnemonic(text);    
  };

  const onPasteToAddr = (e: any) => {
    const text = handlePaste(e);
    setTo(text);
  }

  const onPasteAddress = (e: any) => {
    const addr = handlePaste(e);
    setAddress(addr);
  }

  const handlePaste = (e: any) => {
    // 阻止默认粘贴行为
    e.preventDefault();
    // 获取剪切板数据
    const clipboardData = e.clipboardData || window.clipboardData;
    const pastedText = clipboardData.getData('Text');
    return pastedText;
  }

  const onCreateAccount = () => {
    if(!mnemonic.trim()) {
      message.error('助记词不能为空！');
      return;
    }
    createAccount();
  }

  const onFetchBalance = () => {
    getBalances();
  }

  const onTransfer = () => {
    transfer();
  }

  const onAmountChange = (value: number) => {
    setAmount(value);
  }

  const onCreateClaim = () => {
    createClaim();
  }

  return (
    <PageContainer ghost>
      <div className={styles.container}>
      <Space direction="vertical" size="middle" style={{ display: 'flex' }}>
        <Card title="新建账号">
          <Row gutter={[8, 16]}>
            <Col span={12}>
              <Input placeholder="请输入助记词" onChange={onMnemonicChange} onPaste={onPasteMne} />
            </Col>
            <Col span={4}>
              <Button type="primary" onClick={onCreateAccount} loading={loading}>创建账号</Button>
            </Col>
          </Row>
        </Card>

        <Card title="查询余额">
          <Row gutter={[8, 16]}>
            <Col span={12}>
              <Input placeholder="请输入钱包地址" onPaste={onPasteAddress} value={address}/>
            </Col>
            <Col span={4}>
              <Button type="primary" onClick={onFetchBalance} loading={loading}>查询余额</Button>
            </Col>
            <Col span={8}>
              账户余额为：{`${balances}`}
            </Col>
          </Row>
        </Card>

        <Card title="转账">
          <Row gutter={[8, 16]}>
            <Col span={8}>
            <Form.Item
                label="收款地址"
                name="to"
                rules={[{ required: true, message: '请输入收款人地址!' }]}
              >
                <Input placeholder="请输入收款人地址" onPaste={onPasteToAddr} value={to}/>
              </Form.Item>
            </Col>
            <Col span={6}>
            <Form.Item
                label="转账金额"
                name="amount"
                rules={[{ required: true, message: '请输入转账金额' }]}
              >
                <InputNumber onChange={onAmountChange} />
              </Form.Item>
            </Col>
            <Col span={2}>
              <Button type="primary" onClick={onTransfer} loading={loading}>转账</Button>
            </Col>
          </Row>
        </Card>

        <Card title="链上存证">
          <Row gutter={[8, 16]}>
            <Col span={12}>
              <Input placeholder="请输入存证内容" value={claim} onChange={onClaimChange} />
            </Col>
            <Col span={4}>
              <Button type="primary" onClick={onCreateClaim} loading={loading}>存证</Button>
            </Col>
          </Row>
        </Card>
      </Space>
      </div>
    </PageContainer>
  );
};

export default HomePage;
