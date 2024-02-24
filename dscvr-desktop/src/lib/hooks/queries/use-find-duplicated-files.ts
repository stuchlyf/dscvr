import {useQuery} from "react-query";
import {invoke} from "@tauri-apps/api";
import { DuplicatedFile } from "@/generated/proto/file_indexer";

export default function useFindDuplicatedFiles() {
  return useQuery({
    queryKey: ['duplicated-files'],
    queryFn: async () => {
      return await invoke('find_duplicated_files') as DuplicatedFile[];
    }
  });
}