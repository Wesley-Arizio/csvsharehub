import axios from "axios";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import {ToastContainer, toast, Bounce } from "react-toastify";
import Papa from "papaparse";
import init, {sort_items_by_key} from "./pkg/wasm"

export default function Details() {
    const { id } = useParams();
    const [data, setData] = useState(null);
    const [header, setHeader] = useState(null);
    const [isLoading, setIsLoading] = useState(true);
    const [sortKey, setSortKey] = useState(null);

    useEffect(() => {
        (async () => {
            await init();
        })()
    }, [])

    useEffect(() => {
        if (id) {
            (async ()=> {
                try {
                    const response = await axios.get(`${process.env.REACT_APP_CSV_SHARE_HUB_URL}/csv/${id}`, {
                        responseType: "text"
                    });
                    if (response.status !== 200) {
                        toast.error("Something went wrong. Try again later!");
                        return;
                    }
                    const parsed = Papa.parse(response.data, { header: true, skipEmptyLines: true });
                    setHeader(parsed.meta.fields);
                    setData(parsed.data);
                } catch (e) {
                    if (e.status === 404) {
                        toast.error("File Not Found");
                    } else {
                        toast.error("Internal Server Error");
                    }
                } finally {
                    setIsLoading(false);
                }
    
            })()
        }
    }, [id])

    const sortTable = (e) => {
        try {
            const key = e.target.innerText;
            const sorted = sort_items_by_key(key, data);
            setSortKey(key);
            setData(sorted);
        } catch (e) {
            console.error(e);
        }
    };

    return (
        <div className="App">
            {isLoading && <p>Loading...</p>}
            {!isLoading && data && header && (
                <table className="table table-striped">
                    <thead>
                        <tr>
                            {
                                header.map((header, i) => (
                                    <th key={i} scope="col" onClick={sortTable} className="header-sort">
                                        {header}
                                        {header === sortKey ? <i style={{marginLeft: "20px"}} class="bi bi-arrow-down" /> : <i style={{marginLeft: "20px"}} class="bi bi-arrow-up" /> }
                                    </th>
                                ))
                            }
                        </tr>
                    </thead>
                    <tbody>
                        {
                            data.map((line, i) => (
                            <tr key={i}>
                                {
                                    header.map((key, j) => (
                                        <td key={j}>{line[key]}</td>
                                    ))
                                }
                            </tr>
                            ))
                        }
                    </tbody>
                </table>
            )}
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
    )
}