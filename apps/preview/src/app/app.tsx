// eslint-disable-next-line @typescript-eslint/no-unused-vars
import styles from './app.module.less';
import { VFileMessage } from 'vfile-message';
import { compile } from '@mdx-js/mdx';

export function App() {
  const handleClick = async () => {
    try {
      const res = await compile(`# Hello<>`);
      console.log(res);
    } catch (error) {
      if (error instanceof VFileMessage) {
        console.log(error.message);
      }
    }
  };
  return <button onClick={handleClick}>parse</button>;
}

export default App;
