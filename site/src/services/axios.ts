import axiosImport from "axios";

const axios = axiosImport.create({
    baseURL: import.meta.env.VITE_API_URL
})

export { axios }
