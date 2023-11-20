import { DbInstanceType } from "../App";
import { action, Action, Thunk, thunk } from "easy-peasy";


// export type DbInstanceType = {
//     key: string[];
//     value: ({ KeyPress: string; KeyRelease?: undefined; }
//         | { KeyRelease: string; KeyPress?: undefined; })[];
// }[]
interface DbState {
    dbInstance: DbInstanceType;
    dbCopyInstance: DbInstanceType;
    dbHasChange: boolean;
    dbIsValid: boolean;
}

interface DbActions {
    loadDbInstance: Action<this, DbInstanceType>;
    updateValueCopyDb: Action<this, { key: string[], value: string[] }>;
    updateKeyCopyDb: Action<this, {oldKey: string[], newKey: string[]}>;
    setDbHasChange: Action<this, void>;
    validateDb: Action<this, void>;
}

interface DbThunk {
    // thunkToSetCell: Thunk<this, iCell, undefined, Model>;
    // thunkAddPlayerIds: Thunk<this, string, undefined, Model>;
    // thunkSendMakeMove: Thunk<this, iMakeMove, undefined, Model, Promise<void>>;
}
export interface DbModel extends DbState, DbActions, DbThunk { }


export const dbModel: DbModel = {
    dbInstance: [],
    dbCopyInstance: [],
    dbHasChange: false,
    dbIsValid: false,

    loadDbInstance: action((state, payload) => {
        state.dbInstance = payload;
        state.dbCopyInstance = payload;
    }),
    setDbHasChange: action((state, _) => {
        state.dbHasChange = false;
        if (state.dbInstance.length !== state.dbCopyInstance.length)
            state.dbHasChange = true;
        for (let i = 0; i < state.dbInstance.length; i++) {
            if (state.dbInstance[i].key !== state.dbCopyInstance[i].key)
                state.dbHasChange = true;
            if (state.dbInstance[i].value !== state.dbCopyInstance[i].value)
                state.dbHasChange = true;
        }
    }),
    validateDb: action((state, _) => {
        state.dbIsValid = true;
        for (let i = 0; i < state.dbCopyInstance.length; i++) {
            if (state.dbCopyInstance[i].key.length === 0){
                state.dbIsValid = false;
                console.log("key is empty");
            }
            if (state.dbCopyInstance[i].value.length === 0){
                state.dbIsValid = false;
                console.log("value is empty");
            }
        }
    }),
    updateValueCopyDb: action((state, payload) => {

        const { key, value } = payload;
        const index = state.dbCopyInstance.findIndex((db) => db.key === key);
        if (index !== -1) {
            let newValue = [{ KeyPress: value[0] }, { KeyPress: value[1] },
                            { KeyRelease: value[1] }, { KeyRelease: value[0] }];

            state.dbCopyInstance[index].value = newValue;
        }
    }),
    updateKeyCopyDb: action((state, payload) => {
        const { oldKey, newKey } = payload;
        const index = state.dbCopyInstance.findIndex((db) => db.key === oldKey);
        if (index !== -1) {
            state.dbCopyInstance[index].key = newKey;
        }
        else {
            state.dbCopyInstance.push({ key: newKey, value: [] });
        }
    }),
}
