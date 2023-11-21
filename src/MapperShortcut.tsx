import { ButtonShortcut, MapType } from "./ButtonShortcut"
import "./styles/components.scss"
import { useEffect } from "react"
import { faTrashCan } from "@fortawesome/free-regular-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useStoreActions, useStoreState } from "./store/hook.store";

interface MapperShortcutProps {
    keybind_id: number;
    mapfrom: string;
    mapto: string;
}

export const MapperShortcut = ({ keybind_id, mapfrom, mapto }: MapperShortcutProps) => {


    const { deleteKeybind, validateDb, setDbHasChange } = useStoreActions((actions) => actions.dbModel)

    const { dbInstance, dbCopyInstance, dbHasChange, dbIsValid } = useStoreState((store) => {
        return store.dbModel;
    });
    const deleteHandler = () => {
        deleteKeybind(keybind_id);
        setDbHasChange();
        validateDb();
        console.log(dbHasChange);
    }
    return (
        <div className="mapper-shortcut keybind-holder">
            <ButtonShortcut id={keybind_id} keybind={mapfrom} mapType={MapType.MapFrom} />
            <p>to</p>
            <ButtonShortcut id={keybind_id} keybind={mapto} mapType={MapType.MapTo} />
            <div className="icon-action" onClick={() => deleteHandler()}>
                <FontAwesomeIcon icon={faTrashCan} />
            </div>
        </div>
    )
}
