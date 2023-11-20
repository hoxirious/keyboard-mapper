import { createTypedHooks } from 'easy-peasy';
import { Model } from '../loader/model.loader';


const {useStoreActions,useStoreDispatch,useStoreState,useStore} = createTypedHooks<Model>();

export {useStoreActions,useStoreDispatch,useStoreState,useStore};
