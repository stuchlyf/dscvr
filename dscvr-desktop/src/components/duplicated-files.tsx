import {Button} from "@/components/ui/button.tsx";
import {invoke} from "@tauri-apps/api";
import {useMemo, useState} from "react";
import type { DuplicatedFile } from "@/generated/proto/file_indexer";
import {CopyIcon} from "@radix-ui/react-icons";
import {Accordion, AccordionContent, AccordionItem, AccordionTrigger} from "@/components/ui/accordion.tsx";
import {useQuery} from "react-query";
import LoadingSpinner from "@/components/loading-spinner.tsx";
import useFindDuplicatedFiles from "@/lib/hooks/queries/use-find-duplicated-files.ts";

export default function DuplicatedFiles() {
  const {
    data,
    isError,
    isLoading
  } = useFindDuplicatedFiles();

  return (
    <div>
      {isError && !isLoading && !data && (
        <div>
          {/* TODO: Error state */}
          There was an error while trying to load the duplicated files.
        </div>
      )}
      {!isError && isLoading && !data && (
        <div>
          <LoadingSpinner />
        </div>
      )}
      {!isError && !isLoading && data && (
        <Accordion type={'single'} collapsible>
          {data.map((file) => (
            <DuplicatedFileEntry file={file} key={file.hash} />
          ))}
        </Accordion>
      )}
    </div>
  );
}

export type DuplicatedFileEntryProps = Readonly<{
  file: DuplicatedFile
}>;

function DuplicatedFileEntry({file}: DuplicatedFileEntryProps) {
  const splitFilePath = useMemo(() => {
    return file.paths[0].split('\\');
  }, [file]);

  const fileName = splitFilePath[splitFilePath.length - 1];

  const fileSizeInMib = Math.round(file.aggregatedSize / 1_048_576 * 100) / 100;

  return (
    <AccordionItem value={file.hash}>
      <AccordionTrigger>
        <div className={'flex gap-2 items-center'}>
          <p className={'text-lg font-semibold'}>{fileName} ({fileSizeInMib}MiB)</p>
          <Button variant="ghost" size="icon">
            <CopyIcon className="h-4 w-4"/>
          </Button>
        </div>
      </AccordionTrigger>
      <AccordionContent>
        <p>in {file.paths.length} locations</p>
        <ul>
          {file.paths.map(path => (
            <li key={path}>{path}</li>
          ))}
        </ul>
      </AccordionContent>
    </AccordionItem>
  )
}