// // useState hook
// {
//   function useState(initialValue) {
//     let state = initialValue;
//     const listeners = new Set();

//     const setState = (newValue) => {
//       if (typeof newValue === "function") {
//         state = newValue(state);
//       } else {
//         state = newValue;
//       }
//       listeners.forEach((listener) => listener());
//     };

//     const getState = () => state;

//     const subscribe = (listener) => listeners.add(listener);
//     const unsubscribe = (listener) => listeners.delete(listener);

//     return [getState, setState, subscribe, unsubscribe];
//   }

//   // Usage
//   const [getCount, setCount, subscribe, unsubscribe] = useState(0);

//   subscribe(() => {
//     console.log("State changed:", getCount());
//   });

//   setCount(1); // Logs: "State changed: 1"

//   setCount(2); // Logs: "State changed: 1"
//   setCount(3); // Logs: "State changed: 1"
// }

// // useEffect
// {
//   function useEffect(effect, deps = []) {
//     let hasRunOnce = false;
//     let oldDeps = [];

//     return function () {
//       const hasChanged = deps.some((dep, index) => dep !== oldDeps[index]);

//       if (!hasRunOnce || hasChanged) {
//         if (typeof effect === "function") effect();
//         oldDeps = deps;
//         hasRunOnce = true;
//       }
//     };
//   }
// }

// // Usage
// {
//   const triggerEffect = useEffect(() => {
//     console.log("Effect ran!");
//   }, [1]);

//   triggerEffect(); // Logs: "Effect ran!"
//   triggerEffect(); // Doesn't log since deps haven't changed

//   //useContext
//   function createContext(defaultValue) {
//     const context = { value: defaultValue, listeners: new Set() };

//     return {
//       Provider: {
//         set value(newValue) {
//           context.value = newValue;
//           context.listeners.forEach((listener) => listener(newValue));
//         },
//         get value() {
//           return context.value;
//         },
//       },
//       Consumer: {
//         subscribe: (listener) => context.listeners.add(listener),
//         unsubscribe: (listener) => context.listeners.delete(listener),
//         get value() {
//           return context.value;
//         },
//       },
//     };
//   }
// }
// // Usage
// {
//   const ThemeContext = createContext("light");

//   ThemeContext.Provider.value = "dark";

//   ThemeContext.Consumer.subscribe((value) => {
//     console.log("Theme updated:", value);
//   });

//   ThemeContext.Provider.value = "blue"; // Logs: "Theme updated: blue"
// }
// // useReducer
// {
//   function useReducer(reducer, initialState) {
//     let state = initialState;
//     const listeners = new Set();

//     const dispatch = (action) => {
//       state = reducer(state, action);
//       listeners.forEach((listener) => listener());
//     };

//     const subscribe = (listener) => listeners.add(listener);
//     const unsubscribe = (listener) => listeners.delete(listener);

//     const getState = () => state;

//     return [getState, dispatch, subscribe, unsubscribe];
//   }

//   // Usage
//   function reducer(state, action) {
//     switch (action.type) {
//       case "increment":
//         return state + 1;
//       case "decrement":
//         return state - 1;
//       default:
//         return state;
//     }
//   }

//   const [getState, dispatch, subscribe, unsubscribe] = useReducer(reducer, 0);

//   subscribe(() => {
//     console.log("State updated:", getState());
//   });

//   dispatch({ type: "increment" }); // Logs: "State updated: 1"
//   dispatch({ type: "decrement" }); // Logs: "State updated: 0"
// }

{
  function useState(initialValue) {
    let state = initialValue; // Store the state value

    // The function returned from useState simulates updating the state
    function setState(newValue) {
      // Ensure the state is only updated if the value changes
      if (newValue !== state) {
        state = newValue; // Update the state
        render(); // Trigger a re-render (or some side effect)
      }
    }

    // The getter function returns the current state value
    return [() => state, setState];
  }

  // Example of how to use it:
  let [getState, setState] = useState(0);

  console.log(getState()); // 0 (initial state)
  setState(5);
  console.log(getState()); // 5 (updated state)
  setState(10);
  console.log(getState()); // 10 (updated state)
}
