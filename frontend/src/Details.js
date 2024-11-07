import { useParams } from "react-router-dom";

export default function Details({ params }) {
    const { id } = useParams();
    return (
        <div className="App">
            Hello World {id}!
        </div>
    )
}