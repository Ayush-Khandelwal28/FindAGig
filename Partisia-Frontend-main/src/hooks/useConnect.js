import { useContext } from "react";
import ConnectContext from "../context/ConnectContext";

export default function useConnect() {
  return useContext(ConnectContext);
}