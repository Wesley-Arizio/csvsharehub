import { useState, useRef } from "react";
import axios from "axios";
import { ToastContainer, toast, Bounce } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';
import { Link } from "react-router-dom";
 
function App() {
  const [file, setFile] = useState(null)
  const fileInputRef = useRef(null);
  const [disabled, setDisabled] = useState(true);
  const [list, setList] = useState([]);


  function handleChange(event) {
    setFile(event.target.files[0])
    setDisabled(!disabled);
  }

  const handleSubmit = async (event) => {
    event.preventDefault();
    if (!file) return;
    try {
      const url = `${process.env.REACT_APP_CSV_SHARE_HUB_URL}/csv/`;
      const formData = new FormData();
      formData.append('file', file);
      formData.append('fileName', file.name);
      const config = {
        headers: {
          'content-type': 'multipart/form-data',
        },
      };
      const response = await axios.post(url, formData, config);
      if (response.status !== 200) {
        toast.error("Something went wrong. Try again later!");
        return;
      }
      toast.success('File uploaded successfuly');
      const newItem = {
        id: response.data.id,
        size: response.data.size,
        name: file.name,
      }
      setList(prev => [...prev, newItem]);
      setFile(null);
      setDisabled(true);
      if (fileInputRef.current) {
        fileInputRef.current.value = "";
      }
    } catch (e) {
      const message = e?.message ?? "Internal Server Error"
      toast.error(message);
    }
  }

  return (
    <div className="App">
      <h5 className="card-title">CSV Share Hub</h5>
      <form onSubmit={handleSubmit}>
          <label htmlFor="formFileLg" className="form-label">Upload CSV file here</label>
          <input size={1000} ref={fileInputRef} className="form-control form-control-lg" id="formFileLg" type="file" onChange={handleChange} accept=".csv" />
          <button type="submit" className="btn btn-primary" disabled={disabled}>Upload</button>
      </form>

      <table className="table table-striped">
        <thead>
          <tr>
            <th scope="col">Item</th>
            <th scope="col">File Name</th>
            <th scope="col">Size</th>
            <th scope="col">Link</th>
          </tr>
        </thead>
        <tbody>
          {
            list.map((v, i) => (
              <tr key={v.id}>
                <th scope="row">{i + 1}</th>
                <td>{v.name}</td>
                <td>{v.size}</td>
                <td><Link target="_blank" rel="noopener noreferrer" to={`/details/${v.id}`}>View It</Link></td>
              </tr>
            ))
          }
        </tbody>
      </table>
      <ToastContainer
        position="top-right"
        autoClose={5000}
        hideProgressBar={false}
        newestOnTop={false}
        closeOnClick
        rtl={false}
        pauseOnFocusLoss
        draggable
        pauseOnHover
        theme="light"
        transition={Bounce}
      />
    </div>
  );
}

export default App;
