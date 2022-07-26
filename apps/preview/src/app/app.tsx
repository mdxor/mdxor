// eslint-disable-next-line @typescript-eslint/no-unused-vars
import styles from './app.module.less';

import { Route, Routes } from 'react-router-dom';
import { RealtimePage } from '../pages/realtime';
import { StoragePage } from '../pages/storage';

export function App() {
  return (
    <Routes>
      <Route path="/" element={<RealtimePage />} />
      <Route path="/:storageType/:id" element={<StoragePage />} />
    </Routes>
  );
}

export default App;
