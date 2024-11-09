import axios from "axios";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import {ToastContainer, toast, Bounce } from "react-toastify";
import Papa from "papaparse";

export default function Details() {
    const { id } = useParams();
    const [data, setData] = useState(null);
    const [isLoading, setIsLoading] = useState(true);

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

                    setData(Papa.parse(response.data, { header: true }));
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

    return (
        <div className="App">
            {isLoading && <p>Loading...</p>}
            {!isLoading && data && (
                <table className="table table-striped">
                    <thead>
                        <tr>
                            {
                                data.meta.fields.map((header, i) => (
                                    <th key={i} scope="col">{header}</th>
                                ))
                            }
                        </tr>
                    </thead>
                    <tbody>
                        {
                            data.data.map((line, i) => (
                            <tr key={i}>
                                {
                                    data.meta.fields.map((key, j) => (
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