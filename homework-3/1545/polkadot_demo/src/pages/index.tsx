import { Layout, Flex, Space } from 'antd'
import Head from "next/head";
import App from "../components/App"

export default function Home() {
  return (
    <div>
      <Head>
        <title>Polkadot Wallet</title>
      </Head>
      <Layout>
        <Layout.Header style={{ backgroundColor: '#ccc', textAlign: 'center', fontSize: '24px' }}>
	    Wallet
        </Layout.Header>
        <Layout.Content>
	<div style={{ maxWidth: 1280, margin: '0 auto', padding: 24 }}>
          <App/>
	</div>
        </Layout.Content>
      </Layout>
    </div>
  );
}
