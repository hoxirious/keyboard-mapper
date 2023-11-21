import {createStore} from "easy-peasy"
import {model} from "./model.loader";

const store = createStore(model);

export default store;
