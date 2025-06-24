import "./App.css";
import Navbar from "./components/Navbar.tsx";
import {BrowserRouter, Route, Routes} from "react-router-dom";
import TrainerInfo from "./components/TrainerInfo.tsx";
import Party from "./components/Party.tsx";
import Boxes from "./components/Boxes.tsx";
import Test from "./components/Test.tsx";
import MonDetails from "./components/MonDetails.tsx";

function App() {

  return (
      <BrowserRouter>
          <Navbar />
          <div className="p-4">
              <Routes>
                  <Route path="/trainer" element={<TrainerInfo />} />
                  <Route path="/boxes" element={<Boxes />} />
                  <Route path="/party" element={<Party />} />
                  <Route path={"/test"} element={<Test />} />
                  <Route path="/boxes/:boxId/:storageIndex" element={<MonDetails />} />
              </Routes>
          </div>
      </BrowserRouter>
  );
}

export default App;
