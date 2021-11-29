import { Component, createSignal } from "solid-js";

const App: Component = () => {
  const [clickCount, setClickCount] = createSignal(0);
  return (
    <div class="flex flex-col">
      <p class="text-4xl text-orange-700 text-center py-20">Buenzlimarks</p>
      <p class="text-2xl text-orange-500 text-center py-20">
        A blazingly fast bookmark manager
      </p>
      <button onClick={() => setClickCount(clickCount() + 1)}>
        <p class="text-1xl text-gray-400 text-center py-20">
          You clicked me {clickCount()} times.
        </p>
      </button>
    </div>
  );
};

export default App;
