import { dbModel, DbModel } from "../store/db.model";

export interface Model {
    dbModel: DbModel;
}

export const model: Model = {
    dbModel: dbModel,
}
