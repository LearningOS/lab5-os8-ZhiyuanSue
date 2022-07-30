pub const MATRIX_AXIS :usize=128;
pub const SEMA_BASE:usize=64;
pub struct DL_detect{
    pub row_size:usize,
    pub have_enable_deadlock_detect:bool,
    //vecs
    pub all:[isize;MATRIX_AXIS],
    pub avaliable:[isize;MATRIX_AXIS],
    //matrixes
    //here row is the thread
    //column is the semaphore
    pub allocation:[[isize;MATRIX_AXIS];MATRIX_AXIS],
    pub need:[[isize;MATRIX_AXIS];MATRIX_AXIS],
}

impl DL_detect{
    pub fn new()->Self{
        Self{
            row_size:1, //the main thread
            have_enable_deadlock_detect:false,
            //vecs
            all:[0;MATRIX_AXIS],
            avaliable:[0;MATRIX_AXIS],
            //matrixes
            allocation:[[0;MATRIX_AXIS];MATRIX_AXIS],
            need:[[0;MATRIX_AXIS];MATRIX_AXIS],
        }
    }
    pub fn is_DL_activitive(&self)->bool
    {
        self.have_enable_deadlock_detect
    }
    pub fn add_resource(&mut self,column_id:usize,resource_num:isize)
    {
        self.all[column_id]=resource_num;
        self.avaliable[column_id]=resource_num;
    }
    pub fn detect(&self,column_id:usize) -> bool
    {
        let mut work=[0;MATRIX_AXIS];
        let mut finish=[false;MATRIX_AXIS];
        for i in 0..MATRIX_AXIS{
            work[i]=self.avaliable[i];
        }
        loop{
            let mut find_thread=false;
            for i in 0..self.row_size
            {
                if finish[i]==false
                {
                    let mut all_need_satisify=true;
                    for j in 0..MATRIX_AXIS{
                        if self.need[i][j]>work[j]
                        {
                            all_need_satisify=false;
                            break;
                        }
                    }
                    if all_need_satisify{
                        for j in 0..MATRIX_AXIS
                        {
                            work[j]=work[j]+self.allocation[i][j];
                        }
                        finish[i]=true;
                        find_thread=true;
                        break;
                    }
                }
            }
            if find_thread==false{
                return false;
            }
            let mut all_finish=true;
            for i in 0..self.row_size
            {
                if finish[i]==false{
                    all_finish=false;
                    break;
                }
            }
            if all_finish{
                return true;
            }
        }
    }
    pub fn alloc_lock(&mut self,column_id:usize,row_id:usize)->isize
    {
        self.avaliable[column_id]-=1;
        self.allocation[row_id][column_id]+=1;
        self.need[row_id][column_id]-=1;
        if self.allocation[row_id][column_id]<0
        {
            return -1;
        }
        0
    }
    pub fn free_lock(&mut self,column_id:usize,row_id:usize)->isize
    {
        self.avaliable[column_id]+=1;
        self.allocation[row_id][column_id]-=1;
        0
    }
    pub fn add_need(&mut self,column_id:usize,row_id:usize)
    {
        self.need[row_id][column_id]+=1;
    }
}