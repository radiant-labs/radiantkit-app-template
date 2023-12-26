import { RadiantKitCanvas, RadiantKitProvider } from './core';

function App() {
  return (
    <RadiantKitProvider width={1600} height={1200}>
      <RadiantKitCanvas />
    </RadiantKitProvider>
  );
}

export default App;
