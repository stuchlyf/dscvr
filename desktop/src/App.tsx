import {Tabs, TabsContent, TabsList, TabsTrigger} from "./components/ui/tabs";
import FullTextSearch from "@/components/full-text-search.tsx";
import DuplicatedFiles from "@/components/duplicated-files.tsx";

function App() {
  return (
    <div>
      <div className="container pt-4">
        <Tabs defaultValue="text_search" className={'w-full flex items-center flex-col'}>
          <TabsList>
            <TabsTrigger value="text_search">Full-Text Search</TabsTrigger>
            <TabsTrigger value="duplicated_files">Duplicated Files</TabsTrigger>
          </TabsList>
          <TabsContent value="text_search" className={'w-full'}>
            <FullTextSearch />
          </TabsContent>
          <TabsContent value="duplicated_files" className={'w-full'}>
            <DuplicatedFiles />
          </TabsContent>
        </Tabs>
        </div>
    </div>
  )
}

export default App;
