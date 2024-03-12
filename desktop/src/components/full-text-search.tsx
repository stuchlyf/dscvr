import {Input} from "@/components/ui/input.tsx";
import {invoke} from "@tauri-apps/api";
import {ChangeEvent, MouseEventHandler, useEffect, useState} from "react";
import useDebounce from "@/lib/hooks/use-debounce.ts";
import {cn} from "@/lib/utils.ts";
import {ScrollArea} from "@/components/ui/scroll-area.tsx";
import {Separator} from "@/components/ui/separator.tsx";
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card.tsx";
import {Switch} from "@/components/ui/switch.tsx";
import {Label} from "@/components/ui/label.tsx";
import {ToggleGroup, ToggleGroupItem} from "@/components/ui/toggle-group.tsx";
import {ListBulletIcon, TableIcon} from "@radix-ui/react-icons";

export default function FullTextSearch() {
  const [searchQuery, setSearchQuery] = useState("");
  const [currentPreview, setCurrentPreview] = useState<string>();
  const [results, setResults] = useState<string[]>([]);
  const [view, setView] = useState<'list' | 'table'>('list');
  const [preview, setPreview] = useState(true);
  const searchQueryDebounced = useDebounce(searchQuery, 100);

  const searchForFile = async () => {
      if (!searchQueryDebounced) return;
      setResults(await invoke("search_for_file", { query: searchQueryDebounced }));
  }

  useEffect(() => {
      console.log(searchQueryDebounced)
      searchForFile()
    .then(() => console.log("search done"))
    .catch(e => console.error(e));
  }, [searchQueryDebounced]);

  function handleInputChange(e: ChangeEvent<HTMLInputElement>) {
      setSearchQuery(e.target.value);
  }

  function curryHandleResultClick(result: string): MouseEventHandler<HTMLButtonElement> {
    return function(_) {
      if (currentPreview == result) {
        setCurrentPreview(undefined);
        return;
      }

      setCurrentPreview(result);
    }
  }
    
  return (
    <div>
      <div className={'py-2 flex gap-4 justify-end'}>
        <div className={'flex gap-2 items-center'}>
          <Label htmlFor={'view-picker'}>
            View
          </Label>
          <ToggleGroup type="single" size="sm" value={view} onValueChange={(value) => setView(value as any)}>
            <ToggleGroupItem value="list" aria-label="Toggle view to List">
              <ListBulletIcon className="h-4 w-4" />
            </ToggleGroupItem>
            <ToggleGroupItem value="table" aria-label="Toggle view to table">
              <TableIcon className="h-4 w-4" />
            </ToggleGroupItem>
          </ToggleGroup>
        </div>
        <div className={'flex gap-2 items-center'}>
          <Label htmlFor={'preview-active'}>
            Preview
          </Label>
          <Switch id={'preview-active'} onCheckedChange={(checked: boolean) => setPreview(checked)} />
        </div>
      </div>
      <Input
        placeholder={"Search"}
        onChange={handleInputChange}
        value={searchQuery}
      />
      <Separator className={'my-4'} />
      <div className={'flex gap-8'}>
        <ScrollArea className={'flex-1'}>
          <div className={'flex flex-col gap-2'}>
            {results.map((result, i) => (
              <SearchResult
                key={i}
                path={result}
                currentPreview={currentPreview}
                onClick={curryHandleResultClick(result)}
              />
            ))}
          </div>
        </ScrollArea>
        <div className={'flex-1'}>
          <Card className={'aspect-square'}>
            {!!currentPreview
              ? (
                <>
                  <CardHeader>
                    <CardTitle>
                      {currentPreview}
                    </CardTitle>
                  </CardHeader>
                  <CardContent>
                    <p>TODO</p>
                    <p>{currentPreview}</p>
                  </CardContent>
                </>
              )
              : (
                <CardContent className={'flex justify-center items-center h-full'}>
                  <span className={'text-accent select-none'}>{'no item selected'}</span>
                </CardContent>
              )
            }
          </Card>
        </div>
      </div>
    </div>
  );
}

type SearchResultProps = Readonly<{
  path: string,
  currentPreview?: string,
  onClick: MouseEventHandler<HTMLButtonElement>;
}>

function SearchResult({path, currentPreview, onClick}: SearchResultProps) {
  return (
    <button
      className={cn(
        "flex flex-col items-start gap-2 rounded-lg border p-3 text-left text-sm transition-all hover:bg-accent",
        path === currentPreview && "bg-muted"
      )}
      onClick={onClick}
    >
      <div className="flex w-full flex-col gap-1">
        <div className="flex items-center">
          <div className="flex items-center gap-2">
            <div className="font-semibold break-all">{path}</div>
          </div>
          <div
            className={cn(
              "ml-auto text-xs",
              path === currentPreview
                ? "text-foreground"
                : "text-muted-foreground"
            )}
          >
            size
          </div>
        </div>
      </div>
      <div className="line-clamp-2 text-xs text-muted-foreground">
        lorem ipsum dolor sit amet.
      </div>
    </button>

  )
}