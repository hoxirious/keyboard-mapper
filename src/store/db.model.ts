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
    isRecording: boolean;
}

interface DbActions {
    loadDbInstance: Action<this, DbInstanceType>;
    updateValueCopyDb: Action<this, { index: number, value: string[] }>;
    updateKeyCopyDb: Action<this, { index: number, newKey: string[] }>;
    setDbHasChange: Action<this, void>;
    validateDb: Action<this, void>;
    deleteKeybind: Action<this, number>;
    createKeybind: Action<this, void>;
    setIsRecording: Action<this, boolean>;
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
    isRecording: false,

    setIsRecording: action((state, payload) => {
        state.isRecording = payload;
    }),
    loadDbInstance: action((state, payload) => {
        state.dbInstance = payload;
        state.dbCopyInstance = payload;
    }),
    setDbHasChange: action((state, _) => {
        state.dbHasChange = false;
        if (state.dbInstance.length !== state.dbCopyInstance.length)
            state.dbHasChange = true;
        else
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
            if (state.dbCopyInstance[i].key.length === 0) {
                state.dbIsValid = false;
                console.log("key is empty");
            }
            if (state.dbCopyInstance[i].value.length === 0) {
                state.dbIsValid = false;
                console.log("value is empty");
            }
        }
    }),
    updateValueCopyDb: action((state, payload) => {

        const { index, value } = payload;
        let newValue = [{ KeyPress: value[0] }, { KeyPress: value[1] },
        { KeyRelease: value[1] }, { KeyRelease: value[0] }];

        state.dbCopyInstance[index].value = newValue;
    }),
    updateKeyCopyDb: action((state, payload) => {
        const { index, newKey } = payload;
        if (index !== -1) {
            state.dbCopyInstance[index].key = newKey;
        }
        else {
            state.dbCopyInstance.push({ key: newKey, value: [] });
        }
    }),

    deleteKeybind: action((state, payload) => {
        if (payload)
            state.dbCopyInstance.splice(payload, 1);
    }),

    createKeybind: action((state, _) => {
        state.dbCopyInstance.push({ key: [], value: [] });
    })
}
